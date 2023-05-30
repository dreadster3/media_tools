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
