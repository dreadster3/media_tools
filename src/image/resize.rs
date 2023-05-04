use clap::Args;
use image;

use std::io::Error;

#[derive(Args)]
pub struct ResizeCommand {
    #[clap(short, long)]
    width: u32,

    #[clap(short('H'), long)]
    height: u32,

    #[clap(short, long)]
    output: String,

    #[clap(short('r'), long)]
    keep_ratio: bool,
}

#[derive(Debug)]
pub enum ResizeError {
    IOError(Error),
    ImageCrateError(image::ImageError),
}

impl ResizeCommand {
    pub fn execute(&self, input: &str) -> Result<(), ResizeError> {
        let current_dir = std::env::current_dir().map_err(|e| ResizeError::IOError(e))?;
        let input_path = current_dir.join(input);
        let output_path = current_dir.join(&self.output);

        let img = image::open(input_path).map_err(|e| ResizeError::ImageCrateError(e))?;

        let resized = if self.keep_ratio {
            img.resize(
                self.width,
                self.height,
                image::imageops::FilterType::Nearest,
            )
        } else {
            img.resize_exact(
                self.width,
                self.height,
                image::imageops::FilterType::Nearest,
            )
        };

        resized.save(&self.output).expect("Failed to save image");

        println!("Image saved to {}", output_path.display());

        return Ok(());
    }
}
