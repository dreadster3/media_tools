use clap::{Parser, Subcommand};

use crate::image::cli::{ImageCommand, ImageError};

#[derive(Parser)]
#[command(about, author, version)]
pub struct Cli {
    /// Mode to operate as
    #[clap(subcommand)]
    mode: EMode,

    #[clap(global = true)]
    input: Option<String>,
}

#[derive(Subcommand)]
pub enum EMode {
    /// Image operations
    #[clap(subcommand, name = "image")]
    Image(ImageCommand),
}

#[derive(Debug)]
pub enum CliError {
    ImageError(ImageError),
}

impl Cli {
    pub fn execute(&self) -> Result<(), CliError> {
        match &self.mode {
            EMode::Image(image) => image
                .execute(self.input.as_deref())
                .map_err(|e| CliError::ImageError(e)),
        }
    }
}
