use clap::Subcommand;
use thiserror::Error;

use super::convert::{VideoConvertCommand, VideoConvertError};
use super::watermark::{VideoWatermarkCommand, VideoWatermarkError};

#[derive(Subcommand)]
pub enum VideoCommand {
    /// Convert a video to another format
    #[clap(name = "convert")]
    Convert(VideoConvertCommand),

    /// Add a watermark to a video
    #[clap(name = "watermark")]
    Watermark(VideoWatermarkCommand),
}

#[derive(Debug, Error)]
pub enum VideoError {
    #[error("{0}")]
    ConvertError(VideoConvertError),
    #[error("{0}")]
    WatermarkError(VideoWatermarkError),
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
                VideoCommand::Watermark(watermark) => watermark
                    .execute(&input)
                    .map_err(|e| VideoError::WatermarkError(e)),
            },
            None => Err(VideoError::NoInputError),
        }
    }
}
