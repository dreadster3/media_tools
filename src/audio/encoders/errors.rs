use thiserror::Error;

use super::{mp3, wav};

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Mp3EncodeError(mp3::Mp3EncodeError),
    #[error("{0}")]
    WavEncodeError(wav::WavEncodeError),
    #[error("Function not implemented")]
    NotImplementedError,
}
