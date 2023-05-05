use clap::Args;

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

        let input_extension = input_path.extension().unwrap_or(std::ffi::OsStr::new(""));
        let output_extension = output_path.extension().unwrap_or(input_extension);

        let img = image::open(input_path.clone()).map_err(|e| ConvertError::CrateError(e))?;

        let new_format = match output_extension.to_str() {
            Some("png") => image::ImageFormat::Png,
            Some("jpg") => image::ImageFormat::Jpeg,
            Some("bmp") => image::ImageFormat::Bmp,
            Some("gif") => image::ImageFormat::Gif,
            Some("ico") => image::ImageFormat::Ico,
            Some("tiff") => image::ImageFormat::Tiff,
            Some("webp") => image::ImageFormat::WebP,
            Some("hdr") => image::ImageFormat::Hdr,
            Some("pbm") => image::ImageFormat::Pnm,
            Some("pam") => image::ImageFormat::Pnm,
            Some("ppm") => image::ImageFormat::Pnm,
            Some("pgm") => image::ImageFormat::Pnm,
            Some("pnm") => image::ImageFormat::Pnm,
            _ => return Err(ConvertError::UnsupportedFormat),
        };

        img.save_with_format(&self.output, new_format)
            .map_err(|e| ConvertError::CrateError(e))?;

        return Ok(());
    }
}
