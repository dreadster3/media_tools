use clap::Subcommand;
use log::info;
use thiserror::Error;

use super::blur::{BlurCommand, BlurError};
use super::brightness::{BrightnessCommand, BrightnessError};
use super::convert::{ConvertCommand, ConvertError};
use super::flip::{FlipCommand, FlipError};
use super::resize::{ResizeCommand, ResizeError};
use super::rotate::{RotateCommand, RotateError};
use super::watermark::{WatermarkCommand, WatermarkError};

#[derive(Subcommand)]
pub enum ImageCommand {
    /// Resize an image
    #[clap(name = "resize")]
    Resize(ResizeCommand),

    /// Convert to another image format
    #[clap(name = "convert")]
    Convert(ConvertCommand),

    /// Rotate an image by an arbitrary angle
    #[clap(name = "rotate")]
    Rotate(RotateCommand),

    /// Watermark an image with a smaller image
    #[clap(name = "watermark")]
    Watermark(WatermarkCommand),

    /// Blur an image
    #[clap(name = "blur")]
    Blur(BlurCommand),

    /// Flip an image
    #[clap(name = "flip")]
    Flip(FlipCommand),

    /// Brighten an image
    #[clap(name = "brighten")]
    Brightness(BrightnessCommand),
}

impl std::fmt::Display for ImageCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageCommand::Resize(_) => write!(f, "resize"),
            ImageCommand::Convert(_) => write!(f, "convert"),
            ImageCommand::Rotate(_) => write!(f, "rotate"),
            ImageCommand::Watermark(_) => write!(f, "watermark"),
            ImageCommand::Blur(_) => write!(f, "blur"),
            ImageCommand::Flip(_) => write!(f, "flip"),
            ImageCommand::Brightness(_) => write!(f, "brightness"),
        }
    }
}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("{0}")]
    ResizeError(ResizeError),
    #[error("{0}")]
    ConvertError(ConvertError),
    #[error("{0}")]
    RotateError(RotateError),
    #[error("{0}")]
    WatermarkError(WatermarkError),
    #[error("{0}")]
    BlurError(BlurError),
    #[error("{0}")]
    FlipError(FlipError),
    #[error("{0}")]
    BrightnessError(BrightnessError),
    #[error("No input file provided")]
    NoInputError,
    #[error("Function not implemented")]
    NotImplementedError,
}

impl ImageCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), ImageError> {
        info!("Detected operation: {}", self);

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

                ImageCommand::Blur(blur) => {
                    blur.execute(&input).map_err(|e| ImageError::BlurError(e))
                }

                ImageCommand::Flip(flip) => {
                    flip.execute(&input).map_err(|e| ImageError::FlipError(e))
                }

                ImageCommand::Brightness(brightness) => brightness
                    .execute(&input)
                    .map_err(|e| ImageError::BrightnessError(e)),
            },
            None => Err(ImageError::NoInputError),
        }
    }
}
