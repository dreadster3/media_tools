use clap::Args;
use log::info;
use thiserror::Error;

use crate::internal::utils;

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
    CrateError(image::ImageError),
}

impl FlipCommand {
    pub fn execute(&self, input: &str) -> Result<(), FlipError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let mut img = image::open(input_path.clone()).map_err(|e| FlipError::CrateError(e))?;

        if self.horizontal {
            info!("Flipping horizontally");
            img = img.fliph();
        }

        if self.vertical {
            info!("Flipping vertically");
            img = img.flipv();
        }

        img.save(&output_path)
            .map_err(|e| FlipError::CrateError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
