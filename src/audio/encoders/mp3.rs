use std::io::Write;
use std::{fs, path};

use mp3lame_encoder;

use super::core::Encode;
use super::errors;

pub struct Mp3Encoder {
    file: fs::File,
    writer: mp3lame_encoder::Encoder,
}

#[derive(Debug)]
pub enum Mp3EncodeError {
    Mp3EncoderBuilderError(mp3lame_encoder::BuildError),
    Mp3EncodeError(mp3lame_encoder::EncodeError),
    IoError(std::io::Error),
}

impl Mp3Encoder {
    pub fn new(
        filename: &path::Path,
        channels: u16,
        sample_rate: u32,
    ) -> Result<Self, Mp3EncodeError> {
        let mut builder = mp3lame_encoder::Builder::new().unwrap();

        builder
            .set_num_channels(channels as u8)
            .map_err(|e| Mp3EncodeError::Mp3EncoderBuilderError(e))?;
        builder
            .set_sample_rate(sample_rate)
            .map_err(|e| Mp3EncodeError::Mp3EncoderBuilderError(e))?;
        builder
            .set_quality(mp3lame_encoder::Quality::Best)
            .map_err(|e| Mp3EncodeError::Mp3EncoderBuilderError(e))?;
        builder
            .set_brate(mp3lame_encoder::Bitrate::Kbps192)
            .map_err(|e| Mp3EncodeError::Mp3EncoderBuilderError(e))?;

        let file = fs::File::create(filename).map_err(|e| Mp3EncodeError::IoError(e))?;

        return Ok(Self {
            writer: builder.build().unwrap(),
            file,
        });
    }
}

impl Encode for Mp3Encoder {
    fn encode(&mut self, data: &[f32]) -> Result<(), errors::Error> {
        let interleaved_pcm = mp3lame_encoder::InterleavedPcm { 0: data };

        let mut mp3_out_buffer = Vec::<u8>::new();
        mp3_out_buffer.reserve(mp3lame_encoder::max_required_buffer_size(
            interleaved_pcm.0.len() / 2,
        ));

        let encoded_size = self
            .writer
            .encode(interleaved_pcm, mp3_out_buffer.spare_capacity_mut())
            .map_err(|e| errors::Error::Mp3EncodeError(Mp3EncodeError::Mp3EncodeError(e)))?;

        unsafe {
            mp3_out_buffer.set_len(mp3_out_buffer.len().wrapping_add(encoded_size));
        }

        self.file
            .write(mp3_out_buffer.as_slice())
            .map_err(|e| errors::Error::Mp3EncodeError(Mp3EncodeError::IoError(e)))?;

        return Ok(());
    }
}
