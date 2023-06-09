use std::fmt;

use clap::{Parser, Subcommand};
use log::info;
use thiserror::Error;

use crate::audio::cli::{AudioCommand, AudioError};
use crate::image::cli::{ImageCommand, ImageError};
use crate::video::cli::{VideoCommand, VideoError};

#[derive(Parser)]
#[command(about, author, version)]
pub struct Cli {
    /// Mode to operate as
    #[clap(subcommand)]
    mode: EMode,

    #[arg(global = true)]
    input: Option<String>,
}

#[derive(Subcommand)]
pub enum EMode {
    /// Image operations
    #[clap(subcommand, name = "image")]
    Image(ImageCommand),

    /// Video operations
    #[clap(subcommand, name = "video")]
    Video(VideoCommand),

    /// Audio operations
    #[clap(subcommand, name = "audio")]
    Audio(AudioCommand),
}

impl fmt::Display for EMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EMode::Image(_) => write!(f, "image"),
            EMode::Video(_) => write!(f, "video"),
            EMode::Audio(_) => write!(f, "audio"),
        }
    }
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0}")]
    ImageError(ImageError),
    #[error("{0}")]
    VideoError(VideoError),
    #[error("{0}")]
    AudioError(AudioError),
}

impl Cli {
    pub fn execute(&self) -> Result<(), CliError> {
        info!("Detected mode: {}", self.mode);

        match &self.mode {
            EMode::Image(image) => image
                .execute(self.input.as_deref())
                .map_err(|e| CliError::ImageError(e)),

            EMode::Video(video) => video
                .execute(self.input.as_deref())
                .map_err(|e| CliError::VideoError(e)),

            EMode::Audio(audio) => audio
                .execute(self.input.as_deref())
                .map_err(|e| CliError::AudioError(e)),
        }
    }
}
