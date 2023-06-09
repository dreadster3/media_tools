use thiserror::Error;

use super::{mp3, ogg, wav};

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Mp3EncodeError(mp3::Mp3EncodeError),
    #[error("{0}")]
    WavEncodeError(wav::WavEncodeError),
    #[error("{0}")]
    OggEncodeError(ogg::OggEncoderError),
    #[error("Function not implemented")]
    NotImplementedError,
}
