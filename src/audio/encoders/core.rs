use std::path;

use super::{error, wav};

pub trait Encode {
    fn encode(&mut self, data: &[f32]) -> Result<(), error::Error>;
}

pub fn get_encoder(
    file_path: &path::Path,
    channels: u16,
    sample_rate: u32,
) -> Result<impl Encode, error::Error> {
    if let Some(extension) = file_path.extension() {
        if let Some(extension_str) = extension.to_str() {
            return match extension_str {
                "wav" => Ok(wav::WavEncoder::new(file_path, channels, sample_rate)),
                _ => Err(error::Error::NotImplementedError),
            };
        }
    }

    return Err(error::Error::NotImplementedError);
}
