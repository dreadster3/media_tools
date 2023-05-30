use std::fmt;

use clap::Subcommand;
use log::info;
use thiserror::Error;

use super::convert::{AudioConvertCommand, AudioConvertError};
use super::speed::{AudioSpeedCommand, AudioSpeedError};

#[derive(Subcommand)]
pub enum AudioCommand {
    /// Convert an audio file to another format
    #[clap(name = "convert")]
    Convert(AudioConvertCommand),

    /// Change the speed of an audio file
    #[clap(name = "speed")]
    Speed(AudioSpeedCommand),
}

impl fmt::Display for AudioCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioCommand::Convert(_) => write!(f, "convert"),
            AudioCommand::Speed(_) => write!(f, "speed"),
        }
    }
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("{0}")]
    ConvertError(AudioConvertError),
    #[error("{0}")]
    SpeedError(AudioSpeedError),
    #[error("No input file provided")]
    NoInputError,
}

impl AudioCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), AudioError> {
        info!("Detected operation: {}", self);

        match input {
            Some(input) => match self {
                AudioCommand::Convert(command) => command
                    .execute(input)
                    .map_err(|e| AudioError::ConvertError(e)),
                AudioCommand::Speed(command) => command
                    .execute(input)
                    .map_err(|e| AudioError::SpeedError(e)),
            },
            None => Err(AudioError::NoInputError),
        }
    }
}
