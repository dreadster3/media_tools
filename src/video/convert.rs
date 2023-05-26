use std::process::Command;

use clap::Args;
use log::{debug, info};
use thiserror::Error;

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
    FFmpegError(FFmpegError),
    #[error("Unsupported format")]
    UnsupportedFormat,
}

#[derive(Debug, Error)]
pub enum FFmpegError {
    #[error("Executable not found")]
    ExecutableNotFound,
}

impl VideoConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), VideoConvertError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let normalized_command = utils::normalize_command("ffmpeg");

        debug!("Normalizing command: {}", normalized_command);

        let mut command = Command::new(&normalized_command);

        // Argument handling
        command.arg("-y").arg("-i").arg(&input_path);

        if self.skip_encoding {
            command.arg("-c").arg("copy");
        }

        command.arg(&output_path);

        if let Err(e) = command.output() {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(VideoConvertError::FFmpegError(
                        FFmpegError::ExecutableNotFound,
                    ));
                }
                _ => {
                    return Err(VideoConvertError::IOError(e));
                }
            }
        }

        info!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
