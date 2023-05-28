use std::process::Command;

use clap::Args;
use log::{debug, info};
use thiserror::Error;

use super::ffmpeg::ffmpeg;
use crate::internal::utils;

#[derive(Args)]
pub struct VideoConvertCommand {
    #[clap(short, long)]
    output: String,

    #[clap(short, long)]
    skip_encoding: bool,
}

#[derive(Debug, Error)]
pub enum VideoConvertError {
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("{0}")]
    FFmpegError(ffmpeg::FfmpegError),
    #[error("Unsupported format")]
    UnsupportedFormat,
}

impl VideoConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), VideoConvertError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut builder = ffmpeg::FfmpegCommandBuilder::new();
        builder.input(&input_path).output(&output_path);

        if self.skip_encoding {
            builder.skip_encoding();
        }

        builder
            .execute()
            .map_err(|e| VideoConvertError::FFmpegError(e))?;

        info!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
