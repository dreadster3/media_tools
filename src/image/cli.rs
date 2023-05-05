use clap::Subcommand;

use super::convert::{ConvertCommand, ConvertError};
use super::resize::{ResizeCommand, ResizeError};
use super::rotate::{RotateCommand, RotateError};

#[derive(Subcommand)]
pub enum ImageCommand {
    /// Resize mode
    #[clap(name = "resize")]
    Resize(ResizeCommand),

    /// Convert mode
    #[clap(name = "convert")]
    Convert(ConvertCommand),

    /// Rotate mode
    #[clap(name = "rotate")]
    Rotate(RotateCommand),
}

#[derive(Debug)]
pub enum ImageError {
    ResizeError(ResizeError),
    ConvertError(ConvertError),
    RotateError(RotateError),
    NoInputError,
    NotImplementedError,
}

impl ImageCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), ImageError> {
        match input {
            Some(input) => match self {
                ImageCommand::Resize(resize) => resize
                    .execute(&input)
                    .map_err(|e| ImageError::ResizeError(e)),

                ImageCommand::Convert(convert) => convert
                    .execute(&input)
                    .map_err(|e| ImageError::ConvertError(e)),

                ImageCommand::Rotate(rotate) => rotate
                    .execute(&input)
                    .map_err(|e| ImageError::RotateError(e)),
            },
            None => Err(ImageError::NoInputError),
        }
    }
}
