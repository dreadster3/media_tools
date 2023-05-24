use super::{mp3, wav};

#[derive(Debug)]
pub enum Error {
    Mp3EncodeError(mp3::Mp3EncodeError),
    WavEncodeError(wav::WavEncodeError),
    NotImplementedError,
}
