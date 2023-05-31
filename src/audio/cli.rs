use std::fmt;

use clap::Subcommand;
use log::info;
use thiserror::Error;

use super::boost::{BoostCommand, BoostError};
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

    /// Boost the volume of an audio file
    #[clap(name = "boost")]
    Boost(BoostCommand),
}

impl fmt::Display for AudioCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioCommand::Convert(_) => write!(f, "convert"),
            AudioCommand::Speed(_) => write!(f, "speed"),
            AudioCommand::Boost(_) => write!(f, "boost"),
        }
    }
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("{0}")]
    ConvertError(AudioConvertError),
    #[error("{0}")]
    SpeedError(AudioSpeedError),
    #[error("{0}")]
    BoostError(BoostError),
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
                AudioCommand::Boost(command) => command
                    .execute(input)
                    .map_err(|e| AudioError::BoostError(e)),
            },
            None => Err(AudioError::NoInputError),
        }
    }
}
