use std::fs;

use clap::Args;
use hound::{WavSpec, WavWriter};
use log::{error, info, warn};
use symphonia::core::audio::{RawSample, RawSampleBuffer, SampleBuffer};
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

        let mut sample_count = 0;
        let mut sample_buf = None;
        let mut channels: usize = 0;
        let mut sampling_rate: u32 = 0;
        let mut writer = None;

        loop {
            match format.next_packet() {
                Ok(packet) => {
                    if packet.track_id() != track_id {
                        continue;
                    }

                    match decoder.decode(&packet) {
                        Ok(decoded_packet) => {
                            if sample_buf.is_none() {
                                // Get the audio buffer specification.
                                let spec = *decoded_packet.spec();

                                // Get the capacity of the decoded buffer. Note: This is capacity,
                                // not length!
                                let duration = decoded_packet.capacity() as u64;
                                println!("Duration: {}", duration);

                                channels = spec.channels.count();
                                sampling_rate = spec.rate;

                                // Create the f32 sample buffer.
                                sample_buf = Some(audio::SampleBuffer::<f32>::new(duration, spec));
                                writer = Some(
                                    WavWriter::create(
                                        &output_path,
                                        WavSpec {
                                            channels: channels as u16,
                                            sample_rate: sampling_rate,
                                            bits_per_sample: 32,
                                            sample_format: hound::SampleFormat::Float,
                                        },
                                    )
                                    .unwrap(),
                                );
                            }

                            // Copy the decoded audio buffer into the sample buffer in an
                            // interleaved format.
                            if let Some(buf) = &mut sample_buf {
                                buf.copy_interleaved_ref(decoded_packet);

                                if let Some(w) = &mut writer {
                                    for sample in buf.samples() {
                                        w.write_sample(*sample).unwrap();
                                    }
                                }
                            }
                        }
                        Err(errors::Error::DecodeError(err)) => {
                            warn!("\nDecode error: {}", err);
                        }
                        Err(e) => {
                            return Err(AudioConvertError::SymphoniaError(e));
                        }
                    }
                }
                Err(errors::Error::IoError(err)) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                }
                Err(err) => {
                    error!("Error reading packet: {}", err);
                    return Err(AudioConvertError::SymphoniaError(err));
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
