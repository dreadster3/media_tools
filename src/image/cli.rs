use crate::image::resize::{ResizeCommand, ResizeError};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ImageCommand {
    #[clap(name = "resize")]
    Resize(ResizeCommand),
}

#[derive(Debug)]
pub enum ImageError {
    ResizeError(ResizeError),
    NoInputError,
}

impl ImageCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), ImageError> {
        match &self {
            ImageCommand::Resize(resize) => match input {
                Some(input) => resize
                    .execute(&input)
                    .map_err(|e| ImageError::ResizeError(e)),
                None => Err(ImageError::NoInputError),
            },
        }
    }
}
