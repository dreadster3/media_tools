use image::{ImageBuffer, Pixel};

pub fn get_image_center<P>(img: &ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>) -> (f32, f32)
where
    P: Pixel,
{
    let height = img.height() as f32;
    let width = img.width() as f32;

    return (width / 2f32, height / 2f32);
}

pub fn from_str_to_rgba(rgba_string: &str) -> Result<image::Rgba<u8>, std::num::ParseIntError> {
    let rgba_values: Vec<u8> = rgba_string
        .trim_matches(|p| p == '(' || p == ')')
        .split(',')
        .map(|s| s.trim().parse().unwrap_or_default())
        .collect();

    let color = image::Rgba([
        rgba_values.get(0).unwrap_or(&0).clone(),
        rgba_values.get(1).unwrap_or(&0).clone(),
        rgba_values.get(2).unwrap_or(&0).clone(),
        rgba_values.get(3).unwrap_or(&255).clone(),
    ]);

    return Ok(color);
}

pub fn blend_with_opacity(
    below_color: image::Rgba<u8>,
    overlay_color: image::Rgba<u8>,
    opacity: f32,
) -> image::Rgba<u8> {
    // ( (1-p)R1 + p*R2, (1-p)*G1 + p*G2, (1-p)*B1 + p*B2 )
    let below_color = below_color.channels();
    let overlay_color = overlay_color.channels();

    let alpha_below = overlay_color[3] as f32 / 255f32;

    let opacity = opacity.clamp(0f32, 1f32) * alpha_below;

    let r = ((1f32 - opacity) * below_color[0] as f32 + opacity * overlay_color[0] as f32) as u8;
    let g = ((1f32 - opacity) * below_color[1] as f32 + opacity * overlay_color[1] as f32) as u8;
    let b = ((1f32 - opacity) * below_color[2] as f32 + opacity * overlay_color[2] as f32) as u8;

    return image::Rgba([r, g, b, below_color[3]]);
}
