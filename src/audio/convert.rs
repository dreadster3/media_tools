use clap::Args;
use log::warn;
use symphonia::core::{audio, codecs, errors};
use symphonia::default;
use thiserror::Error;

use super::utils as audio_utils;
use crate::audio::encoders;
use crate::internal::utils;

#[derive(Args)]
pub struct AudioConvertCommand {
    /// Output file
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum AudioConvertError {
    #[error("{0}")]
    AudioFileError(audio_utils::AudioFileError),
    #[error("{0}")]
    ProbeError(audio_utils::ProbeAudioError),
    #[error("{0}")]
    EncodeError(encoders::errors::Error),
}

impl AudioConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), AudioConvertError> {
        let input_path = utils::to_absolute_path(&input);
        let output_path = utils::to_absolute_path(&self.output);

        let audio_file = audio_utils::AudioFile::new(&input_path)
            .map_err(|e| AudioConvertError::AudioFileError(e))?;

        let mut writer = encoders::core::get_encoder(
            &output_path,
            audio_file.channels as u16,
            audio_file.sample_rate,
        )
        .map_err(|e| AudioConvertError::EncodeError(e))?;

        for packet in audio_file {
            if let Ok(samples) = packet {
                writer
                    .encode(&samples)
                    .map_err(|e| AudioConvertError::EncodeError(e))?;
            }
        }

        println!("Audio saved to {}", output_path.display());

        Ok(())
    }
}
