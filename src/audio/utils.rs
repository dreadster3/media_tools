use std::{fs, path};

use symphonia::core::{formats, io, probe};
use symphonia::default;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProbeAudioError {
    #[error("{0}")]
    IoError(std::io::Error),
    #[error("{0}")]
    SymphoniaError(symphonia::core::errors::Error),
}

pub fn probe_audio(audio_file: &path::Path) -> Result<probe::ProbeResult, ProbeAudioError> {
    let mut hint = probe::Hint::new();
    let format_opts = formats::FormatOptions::default();
    let input_file = fs::File::open(audio_file).map_err(|e| ProbeAudioError::IoError(e))?;
    let media_source = io::MediaSourceStream::new(Box::new(input_file), Default::default());

    if let Some(extension) = audio_file.extension() {
        if let Some(extension_str) = extension.to_str() {
            hint.with_extension(&extension_str);
        }
    }

    let probe = default::get_probe()
        .format(&hint, media_source, &format_opts, &Default::default())
        .map_err(|e| ProbeAudioError::SymphoniaError(e))?;

    return Ok(probe);
}

pub fn get_audio_format(
    audio_file: &path::Path,
) -> Result<Box<dyn formats::FormatReader>, ProbeAudioError> {
    let probe = probe_audio(audio_file)?;
    return Ok(probe.format);
}

pub fn interleaved_to_planar(interleaved: &[f32], channels: usize) -> Vec<Vec<f32>> {
    let mut planar = vec![Vec::with_capacity(interleaved.len() / channels); channels];

    for (i, sample) in interleaved.iter().enumerate() {
        planar[i % channels].push(*sample);
    }

    return planar;
}

#[derive(Debug, Error)]
pub enum AudioFileError {
    #[error("{0}")]
    IoError(std::io::Error),
    #[error("{0}")]
    SymphoniaError(symphonia::core::errors::Error),
    #[error("{0}")]
    ProbeError(ProbeAudioError),
    #[error("Sample buffer not initialized")]
    BufferNotInitialized,
}

pub struct AudioFile {
    format: Box<dyn formats::FormatReader>,
    track_id: u32,
    decoder: Box<dyn symphonia::core::codecs::Decoder>,
    sample_buffer: Option<symphonia::core::audio::SampleBuffer<f32>>,

    pub channels: usize,
    pub sample_rate: u32,
}

impl AudioFile {
    pub fn new(file_path: &path::PathBuf) -> Result<Self, AudioFileError> {
        let format = get_audio_format(&file_path).map_err(|e| AudioFileError::ProbeError(e))?;

        let track = format
            .default_track()
            .or_else(|| {
                format
                    .tracks()
                    .iter()
                    .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            })
            .unwrap();

        let track_id = track.id;

        let channels = track.codec_params.channels.unwrap().count();
        let sample_rate = track.codec_params.sample_rate.unwrap();

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &Default::default())
            .map_err(|e| AudioFileError::SymphoniaError(e))?;

        return Ok(Self {
            format,
            track_id,
            decoder,
            sample_buffer: None,
            channels,
            sample_rate,
        });
    }
}

impl Iterator for AudioFile {
    type Item = Result<Vec<f32>, AudioFileError>;

    fn next(&mut self) -> Option<Self::Item> {
        let packet = match self.format.next_packet() {
            Ok(packet) => packet,
            Err(symphonia::core::errors::Error::IoError(err)) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    return None;
                }

                return Some(Err(AudioFileError::IoError(err)));
            }
            Err(e) => return Some(Err(AudioFileError::SymphoniaError(e))),
        };

        if packet.track_id() != self.track_id {
            return self.next();
        }

        let decoded_packet = match self.decoder.decode(&packet) {
            Ok(decoded_packet) => decoded_packet,
            Err(e) => return Some(Err(AudioFileError::SymphoniaError(e))),
        };

        if let None = self.sample_buffer {
            let spec = *decoded_packet.spec();

            let duration = decoded_packet.capacity() as u64;

            let sample_buffer = symphonia::core::audio::SampleBuffer::new(duration, spec);

            self.sample_buffer = Some(sample_buffer);
        }

        if let Some(buf) = self.sample_buffer.as_mut() {
            buf.copy_interleaved_ref(decoded_packet);
            return Some(Ok(buf.samples().to_vec()));
        }

        return Some(Err(AudioFileError::BufferNotInitialized));
    }
}
