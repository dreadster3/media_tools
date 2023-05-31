use clap::Args;
use log::warn;
use symphonia::core::{audio, codecs, errors};
use symphonia::default;
use thiserror::Error;

use super::{encoders, utils as audio_utils};
use crate::internal::utils;

#[derive(Args)]
pub struct BoostCommand {
    /// Boost factor. Note: value will be clamped between 0.0 and infinity
    #[clap(short, long)]
    factor: f32,

    /// Output file
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum BoostError {
    #[error("{0}")]
    IoError(std::io::Error),
    #[error("{0}")]
    SymphoniaError(errors::Error),
    #[error("{0}")]
    ProbeError(audio_utils::ProbeAudioError),
    #[error("{0}")]
    DecodeError(errors::Error),
    #[error("{0}")]
    EncodeError(encoders::errors::Error),
}

impl BoostCommand {
    pub fn execute(&self, input: &str) -> Result<(), BoostError> {
        let input_path = utils::to_absolute_path(&input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut format =
            audio_utils::get_audio_format(&input_path).map_err(|e| BoostError::ProbeError(e))?;

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
            .map_err(|e| BoostError::SymphoniaError(e))?;

        let mut sample_buffer: Option<audio::SampleBuffer<f32>> = None;
        let mut writer = encoders::core::get_encoder(&output_path, channels as u16, sample_rate)
            .map_err(|e| BoostError::EncodeError(e))?;

        loop {
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(errors::Error::IoError(err)) => {
                    if err.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(BoostError::IoError(err));
                    }
                }
                Err(e) => return Err(BoostError::DecodeError(e)),
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
                Err(e) => return Err(BoostError::SymphoniaError(e)),
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

                let boosted_samples = buf
                    .samples()
                    .iter()
                    .map(|s| s * self.factor.clamp(0.0, std::f32::MAX))
                    .collect::<Vec<f32>>();

                writer
                    .encode(&boosted_samples)
                    .map_err(|e| BoostError::EncodeError(e))?;
            }
        }

        println!("Audio saved to {}", output_path.display());

        return Ok(());
    }
}
