use clap::{Args, ValueEnum};
use image::GenericImageView;
use log::{error, info};
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

    /// Opacity of the overlay
    #[clap(short('O'), long, default_value = "1.0")]
    opacity: f32,

    /// Area the overlay should cover
    #[clap(short, long)]
    area: Option<String>,

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

        let mut video_stream = ffmpeg::Ffmpeg::input(0, &input_path);
        let mut watermark_stream = ffmpeg::Ffmpeg::input(1, &watermark_path);
        watermark_stream.scale(50, 50);

        video_stream
            .overlay(&watermark_stream, 0, 0)
            .output(&output_path);

        // info!("Filters: {}", video_stream.compile_filters());

        video_stream.execute();
        Ok(())

        // let watermark =
        //     image::open(&watermark_path).map_err(|e|
        // VideoWatermarkError::ImageError(e))?; let (watermark_width,
        // watermark_height) = watermark.dimensions();
        // let watermark_ratio = watermark_width as f32 / watermark_height as
        // f32;
        //
        // info!(
        //     "Watermark dimensions: {}x{}",
        //     watermark_width, watermark_height
        // );
        //
        // let (video_width, video_height) =
        // ffprobe::get_video_dimensions(&input_path)     .map_err(|e|
        // VideoWatermarkError::ProbeError(e))?;
        //
        // info!("Video dimensions: {}x{}", video_width, video_height);
        //
        // let mut ffmpeg_builder = ffmpeg::FfmpegCommandBuilder::new();
        //
        // ffmpeg_builder
        //     .input(&input_path)
        //     .input(&watermark_path)
        //     .output(&output_path);
        //
        // return Ok(());
    }
}
