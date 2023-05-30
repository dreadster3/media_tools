use std::num::{NonZeroU32, NonZeroU8};
use std::{fs, path};

use thiserror::Error;
use vorbis_rs;

use super::core::Encode;
use super::errors;
use crate::audio::utils;

#[derive(Debug, Error)]
pub enum OggEncoderError {
    #[error("{0}")]
    IOError(std::io::Error),

    #[error("{0}")]
    VorbisError(vorbis_rs::VorbisError),
}

pub struct OggEncoder {
    writer: vorbis_rs::VorbisEncoder<fs::File>,
    channels: u16,
}

impl OggEncoder {
    pub fn new(
        file_path: &path::Path,
        channels: u16,
        sample_rate: u32,
    ) -> Result<Self, OggEncoderError> {
        let file = fs::File::create(file_path).map_err(|e| OggEncoderError::IOError(e))?;

        let encoder = vorbis_rs::VorbisEncoder::new(
            0,
            [("", ""); 0],
            NonZeroU32::new(sample_rate).unwrap(),
            NonZeroU8::new(channels as u8).unwrap(),
            vorbis_rs::VorbisBitrateManagementStrategy::QualityVbr {
                target_quality: 1f32,
            },
            None,
            file,
        )
        .map_err(|e| OggEncoderError::VorbisError(e))?;

        return Ok(Self {
            writer: encoder,
            channels,
        });
    }
}

impl Encode for OggEncoder {
    fn encode(&mut self, data: &[f32]) -> Result<(), errors::Error> {
        let channels_data = utils::interleaved_to_planar(data, self.channels as usize);

        self.writer
            .encode_audio_block(&channels_data)
            .map_err(|e| errors::Error::OggEncodeError(OggEncoderError::VorbisError(e)))?;
        return Ok(());
    }
}
