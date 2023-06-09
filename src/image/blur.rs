use clap::Args;

use thiserror::Error;

use crate::internal::utils;

#[derive(Args)]
pub struct BlurCommand {
    /// Intensity of the blur. Note: The higher the value, the longer it takes
    /// to process. The value will be clamped between 1 and 100.
    #[clap(short, long)]
    intensity: f32,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum BlurError {
    #[error("Image crate error")]
    ImageCrateError(image::ImageError),
}

impl BlurCommand {
    pub fn execute(&self, input: &str) -> Result<(), BlurError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);
        let intensity = self.intensity.clamp(1f32, 100f32);

        let img = image::open(input_path).map_err(|e| BlurError::ImageCrateError(e))?;

        let blurred = img.blur(intensity);

        blurred
            .save(&output_path)
            .map_err(|e| BlurError::ImageCrateError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
