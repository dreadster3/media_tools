use clap::Subcommand;
use thiserror::Error;

use super::convert::{VideoConvertCommand, VideoConvertError};

#[derive(Subcommand)]
pub enum VideoCommand {
    #[clap(name = "convert")]
    Convert(VideoConvertCommand),
}

#[derive(Debug, Error)]
pub enum VideoError {
    #[error("{0}")]
    ConvertError(VideoConvertError),
    #[error("No input file provided")]
    NoInputError,
    #[error("Function not implemented")]
    NotImplementedError,
}

impl VideoCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), VideoError> {
        match input {
            Some(input) => match self {
                VideoCommand::Convert(convert) => convert
                    .execute(&input)
                    .map_err(|e| VideoError::ConvertError(e)),
            },
            None => Err(VideoError::NoInputError),
        }
    }
}
