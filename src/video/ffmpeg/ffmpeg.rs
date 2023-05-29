use std::{path, process};

use log::debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FfmpegError {
    #[error("Executable not found")]
    ExecutableNotFound,
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("Missing output path")]
    MissingOutput,
}

pub struct Ffmpeg {}

impl Ffmpeg {
    pub fn input(id: u32, input: &path::Path) -> FfmpegInputStream {
        return FfmpegInputStream::new(id, input);
    }
}

pub struct FfmpegInputStream {
    id: u32,
    path: path::PathBuf,
    output_path: Option<path::PathBuf>,
    filters: Vec<Filter>,
    arguments: Vec<String>,
    next_result: i32,
}

impl FfmpegInputStream {
    pub fn new(id: u32, path: &path::Path) -> Self {
        return Self {
            id,
            path: path.to_path_buf(),
            arguments: Vec::new(),
            output_path: None,
            filters: Vec::new(),
            next_result: -1,
        };
    }

    fn get_current_result(&self) -> String {
        if self.next_result < 0 {
            return self.id.to_string();
        }

        return format!("r{}{}", self.id, self.next_result);
    }

    fn increment_result(&mut self) -> String {
        self.next_result += 1;
        return self.get_current_result();
    }

    pub fn output(&mut self, output: &path::Path) -> &mut Self {
        self.output_path = Some(output.to_path_buf());
        return self;
    }

    pub fn skip_encoding(&mut self) -> &mut Self {
        self.arguments.push("-c".to_string());
        self.arguments.push("copy".to_string());
        return self;
    }

    pub fn overlay(&mut self, overlay: &FfmpegInputStream, x: u32, y: u32) -> &mut Self {
        let current_result = self.get_current_result();
        let next_result = self.increment_result();

        for filter in overlay.filters.iter() {
            self.arguments.push("-i".to_string());
            self.arguments
                .push(overlay.path.to_str().unwrap().to_string());
            self.filters.push(filter.clone());
        }

        self.filters.push(Filter::overlay(
            current_result,
            overlay.get_current_result(),
            next_result,
            x,
            y,
        ));
        return self;
    }

    pub fn opacity(&mut self, opacity: f32) -> &mut Self {
        let current_result = self.get_current_result();
        let next_result = self.increment_result();

        self.filters
            .push(Filter::opacity(current_result, next_result, opacity));
        return self;
    }

    pub fn scale(&mut self, width: u32, height: u32) -> &mut Self {
        let current_result = self.get_current_result();
        let next_result = self.increment_result();

        self.filters
            .push(Filter::scale(current_result, next_result, width, height));
        return self;
    }

    pub fn compile_filters(&self) -> String {
        return self
            .filters
            .iter()
            .map(|f| f.to_argument())
            .collect::<Vec<String>>()
            .join(";");
    }

    pub fn execute(&mut self) -> Result<(), FfmpegError> {
        let mut command = process::Command::new("ffmpeg");
        command.arg("-i");
        command.arg(&self.path);
        command.args(&self.arguments);

        if self.filters.len() > 0 {
            command.arg("-filter_complex");
            command.arg(self.compile_filters());
            command.arg("-map");
            command.arg(format!("[{}]", self.get_current_result()));
        }

        command.arg("-y");

        return match &self.output_path {
            Some(output) => {
                command.arg(output);

                debug!("Executing: {:?}", command);

                match command.output() {
                    Ok(output) => {
                        debug!("Output: {:?}", output);
                    }

                    Err(e) => match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            return Err(FfmpegError::ExecutableNotFound);
                        }
                        _ => {
                            return Err(FfmpegError::IOError(e));
                        }
                    },
                }

                Ok(())
            }
            None => Err(FfmpegError::MissingOutput),
        };
    }
}

#[derive(Clone)]
struct Filter {
    from: Vec<String>,
    to: String,
    name: String,
    arguments: Vec<String>,
}

impl Filter {
    pub fn scale(from: String, to: String, width: u32, height: u32) -> Self {
        return Self {
            from: vec![from],
            to,
            name: "scale".to_string(),
            arguments: vec![width.to_string(), height.to_string()],
        };
    }

    pub fn overlay(from: String, overlay: String, to: String, x: u32, y: u32) -> Self {
        return Self {
            from: vec![from, overlay],
            to,
            name: "overlay".to_string(),
            arguments: vec![x.to_string(), y.to_string()],
        };
    }

    pub fn opacity(from: String, to: String, opacity: f32) -> Self {
        return Self {
            from: vec![from],
            to,
            name: "format".to_string(),
            arguments: vec![format!("rgba,colorchannelmixer=aa={}", opacity.to_string())],
        };
    }

    pub fn to_argument(&self) -> String {
        let mut argument = String::new();

        let inputs = self
            .from
            .iter()
            .map(|i| format!("[{}]", i))
            .collect::<String>();

        argument.push_str(&inputs);
        argument.push_str(&format!("{}=", self.name));
        argument.push_str(&self.arguments.join(":"));
        argument.push_str(&format!("[{}]", self.to));

        return argument;
    }
}
