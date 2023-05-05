use clap::Subcommand;

use crate::image::convert::{ConvertCommand, ConvertError};
use crate::image::resize::{ResizeCommand, ResizeError};

#[derive(Subcommand)]
pub enum ImageCommand {
    #[clap(name = "resize")]
    Resize(ResizeCommand),

    #[clap(name = "convert")]
    Convert(ConvertCommand),
}

#[derive(Debug)]
pub enum ImageError {
    ResizeError(ResizeError),
    ConvertError(ConvertError),
    NoInputError,
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
            },
            None => Err(ImageError::NoInputError),
        }
    }
}
