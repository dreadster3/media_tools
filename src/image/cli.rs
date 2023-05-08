use clap::Subcommand;

use super::blur::{BlurCommand, BlurError};
use super::convert::{ConvertCommand, ConvertError};
use super::resize::{ResizeCommand, ResizeError};
use super::rotate::{RotateCommand, RotateError};
use super::watermark::{WatermarkCommand, WatermarkError};

#[derive(Subcommand)]
pub enum ImageCommand {
    #[clap(name = "resize")]
    Resize(ResizeCommand),

    #[clap(name = "convert")]
    Convert(ConvertCommand),

    #[clap(name = "rotate")]
    Rotate(RotateCommand),

    #[clap(name = "watermark")]
    Watermark(WatermarkCommand),

    #[clap(name = "blur")]
    Blur(BlurCommand),
}

#[derive(Debug)]
pub enum ImageError {
    ResizeError(ResizeError),
    ConvertError(ConvertError),
    RotateError(RotateError),
    WatermarkError(WatermarkError),
    Blur(BlurError),
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

                ImageCommand::Blur(blur) => blur.execute(&input).map_err(|e| ImageError::Blur(e)),
            },
            None => Err(ImageError::NoInputError),
        }
    }
}
