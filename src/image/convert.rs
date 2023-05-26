use clap::Args;
use log::info;
use thiserror::Error;

use crate::internal::utils;

#[derive(Args)]
pub struct ConvertCommand {
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("{0}")]
    CrateError(image::ImageError),
    #[error("Unsupported format")]
    UnsupportedFormat,
}

impl ConvertCommand {
    pub fn execute(&self, input: &str) -> Result<(), ConvertError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let img = image::open(input_path.clone()).map_err(|e| ConvertError::CrateError(e))?;

        img.save(&output_path)
            .map_err(|e| ConvertError::CrateError(e))?;

        info!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
