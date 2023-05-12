use std::process::Command;

use clap::Args;

use crate::internal::utils;

#[derive(Args)]
pub struct VideoConvertCommand {
    #[clap(short, long)]
    output: String,

    #[clap(short, long)]
    skip_encoding: bool,
}

#[derive(Debug)]
pub enum VideoConvertError {
    IOError(std::io::Error),
    FFmpegError(FFmpegError),
    UnsupportedFormat,
}

#[derive(Debug)]
pub enum FFmpegError {
    UnsupportedFormat,
    ExecutableNotFound,
}

impl VideoConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), VideoConvertError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut command = Command::new(utils::normalize_command("ffmpeg"));

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

        println!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
