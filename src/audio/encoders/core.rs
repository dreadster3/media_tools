use std::path;

use super::{errors, mp3, wav};

pub trait Encode {
    fn encode(&mut self, data: &[f32]) -> Result<(), errors::Error>;
}

pub fn get_encoder(
    file_path: &path::Path,
    channels: u16,
    sample_rate: u32,
) -> Result<Box<dyn Encode>, errors::Error> {
    if let Some(extension) = file_path.extension() {
        if let Some(extension_str) = extension.to_str() {
            return match extension_str {
                "wav" => Ok(Box::new(
                    wav::WavEncoder::new(file_path, channels, sample_rate)
                        .map_err(|e| errors::Error::WavEncodeError(e))?,
                )),
                "mp3" => Ok(Box::new(
                    mp3::Mp3Encoder::new(file_path, channels, sample_rate)
                        .map_err(|e| errors::Error::Mp3EncodeError(e))?,
                )),
                _ => Err(errors::Error::NotImplementedError),
            };
        }
    }

    return Err(errors::Error::NotImplementedError);
}
