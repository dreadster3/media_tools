use clap::Args;
use log::warn;
use symphonia::core::{audio, codecs, errors};
use symphonia::default;
use thiserror::Error;

use super::{encoders, utils as audio_utils};
use crate::internal::utils;

#[derive(Args)]
pub struct BoostCommand {
    /// Boost factor. Values greater than 1.0 will increase the volume, values
    /// less than 1.0 will decrease the volume. Note: value will be clamped
    /// between 0.0 and infinity
    #[clap(short, long)]
    factor: f32,

    /// Output file
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum BoostError {
    #[error("{0}")]
    ProbeError(audio_utils::ProbeAudioError),
    #[error("{0}")]
    AudioFileError(audio_utils::AudioFileError),
    #[error("{0}")]
    EncodeError(encoders::errors::Error),
}

impl BoostCommand {
    pub fn execute(&self, input: &str) -> Result<(), BoostError> {
        let input_path = utils::to_absolute_path(&input);
        let output_path = utils::to_absolute_path(&self.output);

        let audio_file =
            audio_utils::AudioFile::new(&input_path).map_err(|e| BoostError::AudioFileError(e))?;

        let mut writer = encoders::core::get_encoder(
            &output_path,
            audio_file.channels as u16,
            audio_file.sample_rate,
        )
        .map_err(|e| BoostError::EncodeError(e))?;

        for packet in audio_file {
            if let Ok(samples) = packet {
                writer
                    .encode(
                        &samples
                            .iter()
                            .map(|s| s * self.factor)
                            .collect::<Vec<f32>>(),
                    )
                    .map_err(|e| BoostError::EncodeError(e))?;
            }
        }

        println!("Audio saved to {}", output_path.display());

        return Ok(());
    }
}
