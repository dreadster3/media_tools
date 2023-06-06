use clap::Args;

use symphonia::core::{errors};

use thiserror::Error;

use super::{encoders, utils as audio_utils};
use crate::internal::utils;

#[derive(Args)]
pub struct AudioSpeedCommand {
    /// Speed factor (1.0 = normal) (0.5 = half speed) (2.0 = double speed).
    /// Note: Values below 0 will be treated as 0.
    #[clap(short, long)]
    factor: f32,

    /// Output file
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum AudioSpeedError {
    #[error("{0}")]
    IoError(std::io::Error),
    #[error("{0}")]
    SymphoniaError(errors::Error),
    #[error("{0}")]
    ProbeError(audio_utils::ProbeAudioError),
    #[error("{0}")]
    AudioFileError(audio_utils::AudioFileError),
    #[error("{0}")]
    DecodeError(errors::Error),
    #[error("{0}")]
    EncodeError(encoders::errors::Error),
}

impl AudioSpeedCommand {
    pub fn execute(&self, input: &str) -> Result<(), AudioSpeedError> {
        let input_path = utils::to_absolute_path(&input);
        let output_path = utils::to_absolute_path(&self.output);

        let audio_file = audio_utils::AudioFile::new(&input_path)
            .map_err(|e| AudioSpeedError::AudioFileError(e))?;

        let new_sample_rate =
            (audio_file.sample_rate as f32 * self.factor.clamp(0f32, f32::MAX)) as u32;

        let mut writer =
            encoders::core::get_encoder(&output_path, audio_file.channels as u16, new_sample_rate)
                .map_err(|e| AudioSpeedError::EncodeError(e))?;

        for packet in audio_file {
            if let Ok(samples) = packet {
                writer
                    .encode(&samples)
                    .map_err(|e| AudioSpeedError::EncodeError(e))?;
            }
        }

        println!("Audio saved to {}", output_path.display());

        Ok(())
    }
}
