use clap::{Args, ValueEnum};
use log::error;
use thiserror::Error;

use super::ffmpeg::ffprobe;
use crate::internal::utils;
use crate::video::ffmpeg::ffmpeg;

#[derive(Args)]
pub struct VideoWatermarkCommand {
    /// Path to picture to use as overlay
    #[clap(short, long)]
    watermark: String,

    /// Position to place the overlay
    #[clap(short, long, default_value = "center")]
    position: VideoWatermarkPosition,

    /// Opacity of the overlay. Note: value will be clamped between 0.0 and 1.0
    #[clap(short('O'), long, default_value = "1.0")]
    opacity: f32,

    /// The percentage of width the watermark should take up. Note: value will
    /// be clamped between 0.0 and 1.0
    #[clap(short, long)]
    scale: Option<f32>,

    /// Path to output file
    #[clap(short, long)]
    output: String,
}

#[derive(ValueEnum, Clone)]
pub enum VideoWatermarkPosition {
    TopLeft,
    TopRight,
    Center,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Error)]
pub enum VideoWatermarkError {
    #[error("Image error")]
    ImageError(image::ImageError),
    #[error("{0}")]
    ProbeError(ffprobe::FfprobeError),
    #[error("{0}")]
    FfmpegError(ffmpeg::FfmpegError),
    #[error("Dimension error")]
    DimensionError,
}

impl VideoWatermarkCommand {
    pub fn execute(&self, input: &str) -> Result<(), VideoWatermarkError> {
        let input_path = utils::to_absolute_path(input);
        let watermark_path = utils::to_absolute_path(&self.watermark);
        let output_path = utils::to_absolute_path(&self.output);

        let (video_width, video_height) = ffprobe::get_video_dimensions(&input_path)
            .map_err(|e| VideoWatermarkError::ProbeError(e))?;

        let watermark =
            image::open(&watermark_path).map_err(|e| VideoWatermarkError::ImageError(e))?;

        let mut watermark_width = watermark.width();
        let mut watermark_height = watermark.height();
        let watermark_ratio = watermark_width as f32 / watermark_height as f32;

        let mut video_stream = ffmpeg::Ffmpeg::input(0, &input_path);
        let mut watermark_stream = ffmpeg::Ffmpeg::input(1, &watermark_path);
        watermark_stream.opacity(self.opacity.clamp(0f32, 1f32));

        if let Some(scale) = self.scale {
            watermark_width = (video_width as f32 * scale.clamp(0f32, 1f32)) as u32;
            watermark_height = (watermark_width as f32 / watermark_ratio) as u32;
            watermark_stream.scale(watermark_width, watermark_height);
        }

        watermark_width = watermark_width.clamp(0, video_width);
        watermark_height = watermark_height.clamp(0, video_height);

        let mut x = 0;
        let mut y = 0;

        match self.position {
            VideoWatermarkPosition::TopLeft => {}
            VideoWatermarkPosition::TopRight => {
                x = video_width - watermark_width;
            }
            VideoWatermarkPosition::Center => {
                x = (video_width - watermark_width) / 2;
                y = (video_height - watermark_height) / 2;
            }
            VideoWatermarkPosition::BottomLeft => {
                y = video_height - watermark_height;
            }
            VideoWatermarkPosition::BottomRight => {
                x = video_width - watermark_width;
                y = video_height - watermark_height;
            }
        }

        video_stream
            .overlay(&watermark_stream, x, y)
            .output(&output_path);

        return match video_stream.execute() {
            Ok(_) => {
                println!("Video saved to {}", output_path.display());
                Ok(())
            }
            Err(e) => Err(VideoWatermarkError::FfmpegError(e)),
        };
    }
}
