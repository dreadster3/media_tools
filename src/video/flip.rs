use clap::Args;
use thiserror::Error;

use crate::internal::utils;
use crate::video::ffmpeg::ffmpeg;

#[derive(Args)]
pub struct FlipCommand {
    /// Flip the image horizontally
    #[clap(short('H'), long)]
    horizontal: bool,

    /// Flip the image vertically
    #[clap(short, long)]
    vertical: bool,

    /// Output file
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum FlipError {
    #[error("{0}")]
    FFmpegError(ffmpeg::FfmpegError),
}

impl FlipCommand {
    pub fn execute(&self, input: &str) -> Result<(), FlipError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut stream = ffmpeg::Ffmpeg::input(0, &input_path);

        if self.horizontal {
            stream.hflip();
        }

        if self.vertical {
            stream.vflip();
        }

        stream
            .output(&output_path)
            .execute()
            .map_err(|e| FlipError::FFmpegError(e))?;

        println!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
