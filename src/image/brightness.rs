use clap::Args;

use thiserror::Error;

use crate::internal::utils;

#[derive(Args)]
pub struct BrightnessCommand {
    /// Intensity of the brighten.
    /// Positive values will brighten the image, negative values will darken the image.
    /// Note: Value will be clamped between -100 and 100.
    #[clap(short, long)]
    intensity: i32,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum BrightnessError {
    #[error("Image crate error")]
    ImageCrateError(image::ImageError),
}

impl BrightnessCommand {
    pub fn execute(&self, input: &str) -> Result<(), BrightnessError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);
        let intensity = self.intensity.clamp(-100i32, 100i32);

        let img = image::open(input_path).map_err(|e| BrightnessError::ImageCrateError(e))?;

        let brighten_img = img.brighten(intensity);

        brighten_img
            .save(&output_path)
            .map_err(|e| BrightnessError::ImageCrateError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
