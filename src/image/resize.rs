use std::io::Error;

use clap::Args;
use image;

use crate::internal::utils;

#[derive(Args)]
pub struct ResizeCommand {
    /// New width of the image
    #[clap(short, long)]
    width: u32,

    /// Whether the width is a percentage
    #[clap(long)]
    width_as_percentage: bool,

    /// New height of the image
    #[clap(short('H'), long)]
    height: u32,

    /// Whether the height is a percentage
    #[clap(long)]
    height_as_percentage: bool,

    /// Whether to keep the aspect ratio
    #[clap(short('r'), long)]
    keep_ratio: bool,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug)]
pub enum ResizeError {
    IOError(Error),
    ImageCrateError(image::ImageError),
}

impl ResizeCommand {
    pub fn execute(&self, input: &str) -> Result<(), ResizeError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let img = image::open(input_path).map_err(|e| ResizeError::ImageCrateError(e))?;

        let new_height = self.get_new_height(&img);
        let new_width = self.get_new_width(&img);

        let resized = if self.keep_ratio {
            img.resize(new_width, new_height, image::imageops::FilterType::Nearest)
        } else {
            img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest)
        };

        resized
            .save(&self.output)
            .map_err(|e| ResizeError::ImageCrateError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }

    fn get_new_height(&self, img: &image::DynamicImage) -> u32 {
        if self.height_as_percentage {
            let height = img.height();
            return (height as f32 * self.height as f32 / 100.0) as u32;
        }

        return self.height;
    }

    fn get_new_width(&self, img: &image::DynamicImage) -> u32 {
        if self.width_as_percentage {
            let width = img.width();
            return (width as f32 * self.width as f32 / 100.0) as u32;
        }

        return self.width;
    }
}
