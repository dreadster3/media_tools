use clap::Args;
use image::imageops::ColorMap;
use image::{ColorType, DynamicImage, ImageBuffer, Rgba};
use imageproc::geometric_transformations;

use crate::image::utils as image_utils;
use crate::internal::utils;

#[derive(Args)]
pub struct RotateCommand {
    /// Clockwise rotation angle in degrees
    #[clap(short, long)]
    angle: f32,

    /// Whether to perserve the size of the original image
    #[clap(short, long)]
    perserve_size: bool,

    /// Color to fill the image with
    #[clap(short, long)]
    fill_color: Option<String>,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug)]
pub enum RotateError {
    IOError(std::io::Error),
    ImageCrateError(image::ImageError),
    ParseError,
}

impl RotateCommand {
    pub fn execute(&self, input: &str) -> Result<(), RotateError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let img = image::open(input_path).map_err(|e| RotateError::ImageCrateError(e))?;

        let height = img.height();
        let width = img.width();

        let (center_x, center_y) = image_utils::get_image_center(&img.to_rgba8());

        let new_width = if self.perserve_size {
            width
        } else {
            (width as f32 * self.angle.to_radians().cos()
                + height as f32 * self.angle.to_radians().sin())
            .round()
            .abs() as u32
        };

        let new_height = if self.perserve_size {
            height
        } else {
            (width as f32 * self.angle.to_radians().sin()
                + height as f32 * self.angle.to_radians().cos())
            .round()
            .abs() as u32
        };

        let mut buffer = ImageBuffer::from_pixel(new_width, new_height, Rgba([0u8, 0u8, 0u8, 0u8]));

        let (new_center_x, new_center_y) = image_utils::get_image_center(&buffer);

        let projection = geometric_transformations::Projection::translate(-center_x, -center_y)
            .and_then(geometric_transformations::Projection::rotate(
                self.angle.to_radians(),
            ))
            .and_then(geometric_transformations::Projection::translate(
                new_center_x,
                new_center_y,
            ));

        let fill_color = match &self.fill_color {
            Some(color) => {
                image_utils::from_str_to_rgba(&color).map_err(|e| RotateError::ParseError)?
            }
            None => Rgba::from([0u8, 0u8, 0u8, 0u8]),
        };

        geometric_transformations::warp_into(
            &img.to_rgba8(),
            &projection,
            geometric_transformations::Interpolation::Nearest,
            fill_color,
            &mut buffer,
        );

        buffer
            .save(&output_path)
            .map_err(|e| RotateError::ImageCrateError(e))?;

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
