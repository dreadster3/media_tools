use clap::Args;

use thiserror::Error;

use super::ffmpeg::ffmpeg;
use crate::internal::utils;

#[derive(Args)]
pub struct VideoConvertCommand {
    /// Output file
    #[clap(short, long)]
    output: String,

    /// Whether to skip encoding
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

        let mut stream = ffmpeg::Ffmpeg::input(0, &input_path);
        stream.output(&output_path);

        if self.skip_encoding {
            stream.skip_encoding();
        }

        stream
            .execute()
            .map_err(|e| VideoConvertError::FFmpegError(e))?;

        println!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
