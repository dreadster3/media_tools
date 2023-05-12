use clap::Subcommand;

use super::convert::{VideoConvertCommand, VideoConvertError};

#[derive(Subcommand)]
pub enum VideoCommand {
    #[clap(name = "convert")]
    Convert(VideoConvertCommand),
}

#[derive(Debug)]
pub enum VideoError {
    ConvertError(VideoConvertError),
    NoInputError,
    NotImplementedError,
}

impl VideoCommand {
    pub fn execute(&self, input: Option<&str>) -> Result<(), VideoError> {
        match input {
            Some(input) => match self {
                VideoCommand::Convert(convert) => convert
                    .execute(&input)
                    .map_err(|e| VideoError::ConvertError(e)),
            },
            None => Err(VideoError::NoInputError),
        }
    }
}
