use std::path;
use std::process::Command;


use serde_json;
use thiserror::Error;

use crate::internal::utils;

#[derive(Debug, Error)]
pub enum FfprobeError {
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("Executable not found")]
    ExecutableNotFound,
    #[error("{0}")]
    JsonParseError(serde_json::Error),
}

pub fn get_video_dimensions(path: &path::Path) -> Result<(u32, u32), FfprobeError> {
    let mut command = Command::new(utils::normalize_command("ffprobe"));

    command.args(&[
        "-v",
        "error",
        "-select_streams",
        "v:0",
        "-show_entries",
        "stream=width,height",
        "-of",
        "json",
        path.to_str().unwrap(),
    ]);

    return match command.output() {
        Ok(output) => {
            let output = String::from_utf8(output.stdout).unwrap();
            let json: serde_json::Value =
                serde_json::from_str(&output).map_err(|e| FfprobeError::JsonParseError(e))?;

            let width = json["streams"][0]["width"].as_u64().unwrap();
            let height = json["streams"][0]["height"].as_u64().unwrap();

            Ok((width as u32, height as u32))
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                Err(FfprobeError::ExecutableNotFound)
            } else {
                Err(FfprobeError::IOError(e))
            }
        }
    };
}
