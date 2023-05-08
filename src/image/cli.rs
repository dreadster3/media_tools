use clap::Subcommand;

use super::convert::{ConvertCommand, ConvertError};
use super::resize::{ResizeCommand, ResizeError};
use super::rotate::{RotateCommand, RotateError};
use super::watermark::{WatermarkCommand, WatermarkError};

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

    /// Watermark mode
    #[clap(name = "watermark")]
    Watermark(WatermarkCommand),
}

#[derive(Debug)]
pub enum ImageError {
    ResizeError(ResizeError),
    ConvertError(ConvertError),
    RotateError(RotateError),
    WatermarkError(WatermarkError),
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

                ImageCommand::Watermark(watermark) => watermark
                    .execute(&input)
                    .map_err(|e| ImageError::WatermarkError(e)),
            },
            None => Err(ImageError::NoInputError),
        }
    }
}
