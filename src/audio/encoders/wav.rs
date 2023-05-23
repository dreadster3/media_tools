use std::{fs, io, path};

use hound;

use super::encoder::Encode;
use super::error;

#[derive(Debug)]
pub enum WavEncodeError {
    HoundError(hound::Error),
}

pub struct WavEncoder {
    writer: hound::WavWriter<io::BufWriter<fs::File>>,
}

impl WavEncoder {
    pub fn new(filename: &path::Path, channels: u16, sample_rate: u32) -> Self {
        let writer = hound::WavWriter::create(
            filename,
            hound::WavSpec {
                sample_rate,
                channels,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            },
        )
        .expect("Could not create wav writer");

        return Self { writer };
    }
}

impl Encode for WavEncoder {
    fn encode(&mut self, data: &[f32]) -> Result<(), error::Error> {
        for sample in data.iter() {
            match self.writer.write_sample(*sample) {
                Ok(()) => continue,
                Err(e) => WavEncodeError::HoundError(e),
            };
        }

        return Ok(());
    }
}
