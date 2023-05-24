use clap::Subcommand;

use super::convert::{AudioConvertCommand, AudioConvertError};

#[derive(Subcommand)]
pub enum AudioCommand {
    #[clap(name = "convert")]
    Convert(AudioConvertCommand),
}

#[derive(Debug)]
pub enum AudioError {
    ConvertError(AudioConvertError),
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
