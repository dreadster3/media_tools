use clap::Args;
use thiserror::Error;

use super::ffmpeg::ffmpeg;
use crate::internal::utils;

#[derive(Args)]
pub struct MuteCommand {
    /// Output file
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum MuteError {
    #[error("{0}")]
    IOError(std::io::Error),

    #[error("{0}")]
    FFmpegError(ffmpeg::FfmpegError),
}

impl MuteCommand {
    pub fn execute(&self, input: &str) -> Result<(), MuteError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut stream = ffmpeg::Ffmpeg::input(0, &input_path);
        stream.remove_audio().output(&output_path);

        stream.execute().map_err(|e| MuteError::FFmpegError(e))?;

        println!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
