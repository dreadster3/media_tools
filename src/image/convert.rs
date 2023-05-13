use clap::Args;
use log::info;

use crate::internal::utils;

#[derive(Args)]
pub struct ConvertCommand {
    #[clap(short, long)]
    output: String,
}

#[derive(Debug)]
pub enum ConvertError {
    IOError(std::io::Error),
    CrateError(image::ImageError),
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
