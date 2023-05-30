use clap::{Args, ValueEnum};
use image::GenericImageView;
use log::error;
use thiserror::Error;

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

    /// The percentage of width the watermark should take up. Note: value will
    /// be clamped between 0.0 and 1.0
    #[clap(short, long)]
    scale: Option<f32>,

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

#[derive(Debug, Error)]
pub enum WatermarkError {
    #[error("Image crate error")]
    CrateImageError(image::ImageError),
    #[error("Dimension error")]
    DimensionError,
}

impl WatermarkCommand {
    pub fn execute(&self, input: &str) -> Result<(), WatermarkError> {
        let input_path = utils::to_absolute_path(input);
        let watermark_path = utils::to_absolute_path(&self.watermark);
        let output_path = utils::to_absolute_path(&self.output);

        let mut img = image::open(&input_path).map_err(|e| WatermarkError::CrateImageError(e))?;
        let (image_width, image_height) = img.dimensions();

        let mut watermark =
            image::open(&watermark_path).map_err(|e| WatermarkError::CrateImageError(e))?;

        if let Some(scale) = self.scale {
            let (width, height) = watermark.dimensions();
            let watermark_ratio = width as f32 / height as f32;
            let new_width = (image_width as f32 * scale).round() as u32;
            let new_height = (new_width as f32 / watermark_ratio).round() as u32;

            watermark =
                watermark.resize(new_width, new_height, image::imageops::FilterType::Nearest);
        }

        let (watermark_width, watermark_height) = watermark.dimensions();

        if watermark_width > image_width || watermark_height > image_height {
            error!("Watermark is bigger than the image");
            return Err(WatermarkError::DimensionError);
        }

        let (x, y) = match self.position {
            WatermarkPosition::TopLeft => (0, 0),
            WatermarkPosition::TopRight => (image_width - watermark_width, 0),
            WatermarkPosition::BottomLeft => (0, image_height - watermark_height),
            WatermarkPosition::BottomRight => (
                image_width - watermark_width,
                image_height - watermark_height,
            ),
            WatermarkPosition::Center => (
                image_width / 2 - (watermark_width / 2),
                image_height / 2 - (watermark_height / 2),
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
