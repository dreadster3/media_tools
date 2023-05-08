use clap::{Args, ValueEnum};
use image::GenericImageView;

use super::utils as image_utils;
use crate::internal::utils;

#[derive(Args)]
pub struct WatermarkCommand {
    /// Path to picture to use as overlay
    #[clap(short, long)]
    watermark: String,

    /// Position to place the overlay
    #[clap(short, long, default_value = "center")]
    position: WatermarkPosition,

    /// Opacity of the overlay
    #[clap(short('O'), long, default_value = "1.0")]
    opacity: f32,

    /// Path to output file
    #[clap(short, long)]
    output: String,
}

#[derive(ValueEnum, Clone)]
pub enum WatermarkPosition {
    TopLeft,
    TopRight,
    Center,
    BottomLeft,
    BottomRight,
}

#[derive(Debug)]
pub enum WatermarkError {
    CrateImageError(image::ImageError),
    DimensionError,
}

impl WatermarkCommand {
    pub fn execute(&self, input: &str) -> Result<(), WatermarkError> {
        let input_path = utils::to_absolute_path(input);
        let watermark_path = utils::to_absolute_path(&self.watermark);
        let output_path = utils::to_absolute_path(&self.output);

        let watermark =
            image::open(&watermark_path).map_err(|e| WatermarkError::CrateImageError(e))?;
        let (watermark_width, watermark_height) = watermark.dimensions();

        let mut img = image::open(&input_path).map_err(|e| WatermarkError::CrateImageError(e))?;
        let (width, height) = img.dimensions();

        if watermark_width > width || watermark_height > height {
            println!("Watermark is bigger than the image");
            return Err(WatermarkError::DimensionError);
        }

        let (x, y) = match self.position {
            WatermarkPosition::TopLeft => (0, 0),
            WatermarkPosition::TopRight => (width - watermark_width, 0),
            WatermarkPosition::BottomLeft => (0, height - watermark_height),
            WatermarkPosition::BottomRight => (width - watermark_width, height - watermark_height),
            WatermarkPosition::Center => (
                width / 2 - (watermark_width / 2),
                height / 2 - (watermark_height / 2),
            ),
        };

        let region_of_interest = img.crop(x, y, watermark_width, watermark_height);
        let mut transparent_watermark = watermark.to_rgba8();

        for pixel in transparent_watermark
            .pixels_mut()
            .zip(region_of_interest.pixels())
        {
            let (watermark_pixel, roi_pixel) = pixel;

            let combined_color =
                image_utils::blend_with_opacity(roi_pixel.2, *watermark_pixel, self.opacity);

            *watermark_pixel = combined_color;
        }

        image::imageops::overlay(&mut img, &transparent_watermark, x as i64, y as i64);

        img.save(&output_path)
            .map_err(|e| WatermarkError::CrateImageError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
