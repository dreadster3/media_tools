use clap::Args;
use thiserror::Error;

use super::ffmpeg::ffmpeg;
use super::utils as video_utils;
use crate::internal::utils;
use crate::video::ffmpeg::ffprobe;

#[derive(Args)]
pub struct RotateCommand {
    /// Clockwise rotation angle in degrees
    #[clap(short, long)]
    angle: f32,

    /// Whether to perserve the size of the original video
    #[clap(short, long)]
    perserve_size: bool,

    /// Color to fill the video with after rotation. Format: (r, g, b, a)
    #[clap(short, long)]
    fill_color: Option<String>,

    /// Output path
    #[clap(short, long)]
    output: String,
}

#[derive(Debug, Error)]
pub enum RotateError {
    #[error("{0}")]
    ProbeError(ffprobe::FfprobeError),

    #[error("{0}")]
    FfmpegError(ffmpeg::FfmpegError),
}

impl RotateCommand {
    pub fn execute(&self, input: &str) -> Result<(), RotateError> {
        let input_path = utils::to_absolute_path(input);
        let output_path = utils::to_absolute_path(&self.output);

        let (video_width, video_height) =
            ffprobe::get_video_dimensions(&input_path).map_err(|e| RotateError::ProbeError(e))?;

        let new_width = if self.perserve_size {
            video_width
        } else {
            (video_width as f32 * self.angle.to_radians().cos()
                + video_height as f32 * self.angle.to_radians().sin())
            .round()
            .abs() as u32
        };

        let new_height = if self.perserve_size {
            video_height
        } else {
            (video_width as f32 * self.angle.to_radians().sin()
                + video_height as f32 * self.angle.to_radians().cos())
            .round()
            .abs() as u32
        };

        let mut video = ffmpeg::Ffmpeg::input(0, &input_path);

        let fill_color = match self.fill_color {
            Some(ref color) => video_utils::from_str_to_hex(color).unwrap(),
            None => String::from("#000000"),
        };

        if self.perserve_size {
            video.rotate(self.angle);
        } else {
            video
                .pad(
                    new_width.max(video_width),
                    new_height.max(video_height),
                    &fill_color,
                )
                .rotate(self.angle)
                .crop(new_width, new_height);
        }

        video.output(&output_path);

        video.execute().map_err(|e| RotateError::FfmpegError(e))?;

        println!("Video saved to {}", output_path.display());

        return Ok(());
    }
}
