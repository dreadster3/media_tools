use clap::Args;
use image::Rgba;
use imageproc::geometric_transformations;

use crate::internal::utils;

#[derive(Args)]
pub struct RotateCommand {
    /// Clockwise rotation angle in degrees
    #[clap(short, long)]
    angle: f32,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug)]
pub enum RotateError {
    IOError(std::io::Error),
    ImageCrateError(image::ImageError),
}

impl RotateCommand {
    pub fn execute(&self, input: &str) -> Result<(), RotateError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let img = image::open(input_path).map_err(|e| RotateError::ImageCrateError(e))?;

        let rotated = geometric_transformations::rotate_about_center(
            &img.to_rgba8(),
            self.angle.to_radians(),
            geometric_transformations::Interpolation::Nearest,
            Rgba::from([0u8, 0u8, 0u8, 0u8]),
        );

        rotated
            .save(&output_path)
            .map_err(|e| RotateError::ImageCrateError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
