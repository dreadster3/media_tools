use std::{fs, io, path};

use hound;

use super::core::Encode;
use super::errors;

#[derive(Debug)]
pub enum WavEncodeError {
    HoundError(hound::Error),
}

pub struct WavEncoder {
    writer: hound::WavWriter<io::BufWriter<fs::File>>,
}

impl WavEncoder {
    pub fn new(
        filename: &path::Path,
        channels: u16,
        sample_rate: u32,
    ) -> Result<Self, WavEncodeError> {
        let writer = hound::WavWriter::create(
            filename,
            hound::WavSpec {
                sample_rate,
                channels,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            },
        )
        .map_err(|e| WavEncodeError::HoundError(e))?;

        return Ok(Self { writer });
    }
}

impl Encode for WavEncoder {
    fn encode(&mut self, data: &[f32]) -> Result<(), errors::Error> {
        for sample in data.iter() {
            self.writer
                .write_sample(*sample)
                .map_err(|e| errors::Error::WavEncodeError(WavEncodeError::HoundError(e)))?;
        }

        return Ok(());
    }
}
