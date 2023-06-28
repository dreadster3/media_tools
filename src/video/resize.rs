use clap::Args;
use log::info;
use thiserror::Error;

use super::ffmpeg::{ffmpeg, ffprobe};
use crate::internal::utils;

#[derive(Args)]
pub struct ResizeCommand {
    /// New width of the video
    #[clap(short, long)]
    width: u32,

    /// Whether the width is a percentage
    #[clap(long)]
    width_as_percentage: bool,

    /// New height of the video
    #[clap(short('H'), long)]
    height: u32,

    /// Whether the height is a percentage
    #[clap(long)]
    height_as_percentage: bool,

    /// Whether to keep the aspect ratio
    #[clap(short('r'), long)]
    keep_ratio: bool,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum ResizeError {
    #[error("{0}")]
    ProbeError(ffprobe::FfprobeError),
    #[error("{0}")]
    FfmpegError(ffmpeg::FfmpegError),
}

impl ResizeCommand {
    pub fn execute(&self, input: &str) -> Result<(), ResizeError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let (video_width, video_height) =
            ffprobe::get_video_dimensions(&input_path).map_err(|e| ResizeError::ProbeError(e))?;

        let ratio = video_width as f32 / video_height as f32;
        let mut new_width = self.get_new_width(video_width);
        let mut new_height = self.get_new_height(video_height);

        let mut stream = ffmpeg::Ffmpeg::input(0, &input_path);

        if self.keep_ratio {
            let mut possible_width = (new_height as f32 * ratio).round() as u32;
            let mut possible_height = (new_width as f32 / ratio).round() as u32;

            let possible_width_difference = (new_width as i32 - possible_width as i32).abs();
            let possible_height_difference = (new_height as i32 - possible_height as i32).abs();

            if possible_width_difference < possible_height_difference {
                if possible_width % 2 != 0 {
                    possible_width += 1;
                }

                new_width = possible_width;
            } else {
                if possible_height % 2 != 0 {
                    possible_height += 1;
                }

                new_height = possible_height;
            }
        }

        info!(
            "Resizing video to {}x{} (original: {}x{})",
            new_width, new_height, video_width, video_height
        );

        stream.scale(new_width, new_height).output(&output_path);

        stream.execute().map_err(|e| ResizeError::FfmpegError(e))?;

        println!("Video saved to {}", output_path.display());

        return Ok(());
    }

    pub fn get_new_height(&self, video_height: u32) -> u32 {
        if self.height_as_percentage {
            return (video_height as f32 * self.height.clamp(0, 100) as f32 / 100.0).round() as u32;
        }

        return self.height;
    }

    pub fn get_new_width(&self, video_width: u32) -> u32 {
        if self.width_as_percentage {
            return (video_width as f32 * self.width.clamp(0, 100) as f32 / 100.0).round() as u32;
        }

        return self.width;
    }
}
