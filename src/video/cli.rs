use clap::Subcommand;
use log::info;
use thiserror::Error;

use super::convert::{VideoConvertCommand, VideoConvertError};
use super::mute::{MuteCommand, MuteError};
use super::resize::{ResizeCommand, ResizeError};
use super::rotate::{RotateCommand, RotateError};
use super::watermark::{VideoWatermarkCommand, VideoWatermarkError};

#[derive(Subcommand)]
pub enum VideoCommand {
    /// Convert a video to another format
    #[clap(name = "convert")]
    Convert(VideoConvertCommand),

    /// Add a watermark to a video
    #[clap(name = "watermark")]
    Watermark(VideoWatermarkCommand),

    /// Remove the audio from a video
    #[clap(name = "mute")]
    Mute(MuteCommand),

    /// Resize a video
    #[clap(name = "resize")]
    Resize(ResizeCommand),

    /// Rotate a video
    #[clap(name = "rotate")]
    Rotate(RotateCommand),
}

impl std::fmt::Display for VideoCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoCommand::Convert(_) => write!(f, "convert"),
            VideoCommand::Watermark(_) => write!(f, "watermark"),
            VideoCommand::Mute(_) => write!(f, "mute"),
            VideoCommand::Rotate(_) => write!(f, "rotate"),
            VideoCommand::Resize(_) => write!(f, "resize"),
        }
    }
}

#[derive(Debug, Error)]
pub enum VideoError {
    #[error("{0}")]
    ConvertError(VideoConvertError),
    #[error("{0}")]
    WatermarkError(VideoWatermarkError),
    #[error("{0}")]
    MuteError(MuteError),
    #[error("{0}")]
    RotateError(RotateError),
    #[error("{0}")]
    ResizeError(ResizeError),
    #[error("No input file provided")]
    NoInputError,
    #[error("Function not implemented")]
    NotImplementedError,
}

impl VideoCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), VideoError> {
        info!("Detected operation: {}", self);

        match input {
            Some(input) => match self {
                VideoCommand::Convert(convert) => convert
                    .execute(&input)
                    .map_err(|e| VideoError::ConvertError(e)),
                VideoCommand::Watermark(watermark) => watermark
                    .execute(&input)
                    .map_err(|e| VideoError::WatermarkError(e)),
                VideoCommand::Mute(mute) => {
                    mute.execute(&input).map_err(|e| VideoError::MuteError(e))
                }
                VideoCommand::Rotate(rotate) => rotate
                    .execute(&input)
                    .map_err(|e| VideoError::RotateError(e)),
                VideoCommand::Resize(resize) => resize
                    .execute(&input)
                    .map_err(|e| VideoError::ResizeError(e)),
            },
            None => Err(VideoError::NoInputError),
        }
    }
}
