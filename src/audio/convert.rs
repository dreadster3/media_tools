use std::fs::{self, File};
use std::io::BufWriter;

use clap::Args;
use hound;
use log::{error, info, warn};
use symphonia::core::{audio, errors, formats, io, probe};
use symphonia::default;

use crate::internal::utils;

#[derive(Args)]
pub struct AudioConvertCommand {
    #[clap(short, long)]
    output: String,
}

#[derive(Debug)]
pub enum AudioConvertError {
    IoError(std::io::Error),
    SymphoniaError(errors::Error),
    DecodeError(errors::Error),
    HoundError(hound::Error),
}

impl AudioConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), AudioConvertError> {
        let input_path = utils::to_absolute_path(&input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut hint = probe::Hint::new();
        let format_opts = formats::FormatOptions::default();
        let input_file = fs::File::open(&input_path).map_err(|e| AudioConvertError::IoError(e))?;
        let media_source = io::MediaSourceStream::new(Box::new(input_file), Default::default());

        if let Some(extension) = input_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(&extension_str);
            }
        }

        let probe = default::get_probe()
            .format(&hint, media_source, &format_opts, &Default::default())
            .map_err(|e| AudioConvertError::SymphoniaError(e))?;

        let mut format = probe.format;

        let track = format.default_track().unwrap();

        let track_id = track.id;

        let mut decoder = default::get_codecs()
            .make(&track.codec_params, &Default::default())
            .unwrap();

        // println!(
        //     "Bits per sample: {}",
        //     track.codec_params.bits_per_sample.unwrap()
        // );
        println!("Sample Rate: {}", track.codec_params.sample_rate.unwrap());

        // let mut sample_count = 0;
        // let mut sample_buf = None;
        // let mut channels: usize = 0;
        // let mut sampling_rate: u32 = 0;
        let mut writer: Option<hound::WavWriter<BufWriter<File>>> = None;
        let mut sample_buffer: Option<audio::SampleBuffer<f32>> = None;

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(errors::Error::IoError(err)) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(AudioConvertError::IoError(err));
                    }
                }
                Err(e) => return Err(AudioConvertError::DecodeError(e)),
            };

            if packet.track_id() != track_id {
                continue;
            }

            let decoded_packet = match decoder.decode(&packet) {
                Ok(decoded_packet) => decoded_packet,
                Err(errors::Error::DecodeError(err)) => {
                    warn!("Decode error: {}", err);
                    continue;
                }
                Err(e) => return Err(AudioConvertError::SymphoniaError(e)),
            };

            sample_buffer = match sample_buffer {
                Some(sample_buffer) => Some(sample_buffer),
                None => {
                    let spec = *decoded_packet.spec();

                    let duration = decoded_packet.capacity() as u64;

                    let sample_buffer = audio::SampleBuffer::new(duration, spec);

                    Some(sample_buffer)
                }
            };

            writer = match writer {
                Some(writer) => Some(writer),
                None => {
                    let spec = *decoded_packet.spec();

                    let channels = spec.channels.count();
                    let sampling_rate = spec.rate;

                    let writer = hound::WavWriter::create(
                        &output_path,
                        hound::WavSpec {
                            channels: channels as u16,
                            sample_rate: sampling_rate,
                            bits_per_sample: 32,
                            sample_format: hound::SampleFormat::Float,
                        },
                    )
                    .map_err(|e| AudioConvertError::HoundError(e))?;

                    Some(writer)
                }
            };

            if let Some(buf) = sample_buffer.as_mut() {
                buf.copy_interleaved_ref(decoded_packet);

                if let Some(w) = writer.as_mut() {
                    for sample in buf.samples() {
                        w.write_sample(*sample).unwrap();
                    }
                }
            }
        }

        if let Some(w) = writer {
            w.finalize().unwrap();
        }

        info!(
            "Converting audio from {} to {}",
            input_path.display(),
            output_path.display()
        );

        Ok(())
    }
}
