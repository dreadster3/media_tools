use std::fs;

use clap::Args;
use log::{info, warn};
use symphonia::core::{audio, codecs, errors, formats, io as symphonia_io, probe};
use symphonia::default;

use crate::audio::encoders;
use crate::audio::encoders::core::Encode;
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
    EncodeError(encoders::errors::Error),
}

impl AudioConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), AudioConvertError> {
        let input_path = utils::to_absolute_path(&input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut hint = probe::Hint::new();
        let format_opts = formats::FormatOptions::default();
        let input_file = fs::File::open(&input_path).map_err(|e| AudioConvertError::IoError(e))?;
        let media_source =
            symphonia_io::MediaSourceStream::new(Box::new(input_file), Default::default());

        if let Some(extension) = input_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(&extension_str);
            }
        }

        let probe = default::get_probe()
            .format(&hint, media_source, &format_opts, &Default::default())
            .map_err(|e| AudioConvertError::SymphoniaError(e))?;

        let mut format = probe.format;

        // Default track or find the first non-null track
        let track = format
            .default_track()
            .or_else(|| {
                format
                    .tracks()
                    .iter()
                    .find(|t| t.codec_params.codec != codecs::CODEC_TYPE_NULL)
            })
            .unwrap();

        let track_id = track.id;

        let channels = track.codec_params.channels.unwrap().count();
        let sample_rate = track.codec_params.sample_rate.unwrap();
        let mut decoder = default::get_codecs()
            .make(&track.codec_params, &Default::default())
            .map_err(|e| AudioConvertError::SymphoniaError(e))?;

        // let mut writer = hound::WavWriter::create(
        //     &output_path,
        //     hound::WavSpec {
        //         channels: channels as u16,
        //         sample_rate: sampling_rate,
        //         bits_per_sample: 32,
        //         sample_format: hound::SampleFormat::Float,
        //     },
        // )
        // .map_err(|e| AudioConvertError::HoundError(e))?;
        // let mut mp3_encoder_builder = mp3lame_encoder::Builder::new().unwrap();
        // mp3_encoder_builder
        //     .set_num_channels(channels as u8)
        //     .unwrap();
        // mp3_encoder_builder.set_sample_rate(sampling_rate).unwrap();
        // mp3_encoder_builder
        //     .set_brate(mp3lame_encoder::Birtate::Kbps192)
        //     .unwrap();
        // mp3_encoder_builder
        //     .set_quality(mp3lame_encoder::Quality::Best)
        //     .unwrap();
        // let mut mp3_encoder = mp3_encoder_builder.build().unwrap();
        // let mut file_test = fs::File::create("test.mp3").unwrap();

        let mut sample_buffer: Option<audio::SampleBuffer<f32>> = None;
        let mut writer = encoders::core::get_encoder(&output_path, channels as u16, sample_rate)
            .map_err(|e| AudioConvertError::EncodeError(e))?;

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

            if let Some(buf) = sample_buffer.as_mut() {
                buf.copy_interleaved_ref(decoded_packet);

                writer
                    .encode(buf.samples())
                    .map_err(|e| AudioConvertError::EncodeError(e))?;

                // let data = InterleavedPcm { 0: buf.samples() };
                //
                // let mut mp3_out_buffer = Vec::<u8>::new();
                // mp3_out_buffer.
                // reserve(mp3lame_encoder::max_required_buffer_size(data.0.
                // len() / 2));
                //
                // let encoded_size = mp3_encoder
                //     .encode(data, mp3_out_buffer.spare_capacity_mut())
                //     .unwrap();
                // unsafe {
                //     mp3_out_buffer.set_len(mp3_out_buffer.len().
                // wrapping_add(encoded_size)); }
                //
                // let encoded_size = mp3_encoder
                //     .flush::<mp3lame_encoder::FlushNoGap>(mp3_out_buffer.
                // spare_capacity_mut())     .unwrap();
                // unsafe {
                //     mp3_out_buffer.set_len(mp3_out_buffer.len().
                // wrapping_add(encoded_size)); }
                //
                // file_test.write(mp3_out_buffer.as_slice()).unwrap();
            }
        }

        info!(
            "Converting audio from {} to {}",
            input_path.display(),
            output_path.display()
        );

        Ok(())
    }
}
