use clap::Subcommand;
use thiserror::Error;

use super::convert::{AudioConvertCommand, AudioConvertError};

#[derive(Subcommand)]
pub enum AudioCommand {
    #[clap(name = "convert")]
    Convert(AudioConvertCommand),
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("{0}")]
    ConvertError(AudioConvertError),
    #[error("No input file provided")]
    NoInputError,
}

impl AudioCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), AudioError> {
        match input {
            Some(input) => match self {
                AudioCommand::Convert(command) => command
                    .execute(input)
                    .map_err(|e| AudioError::ConvertError(e)),
            },
            None => Err(AudioError::NoInputError),
        }
    }
}
