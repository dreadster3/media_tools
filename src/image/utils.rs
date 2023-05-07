use image::{ImageBuffer, Pixel};

pub fn get_image_center<P>(img: &ImageBuffer<P, Vec<<P as Pixel>::Subpixel>>) -> (f32, f32)
where
    P: Pixel,
{
    let height = img.height() as f32;
    let width = img.width() as f32;
    (width / 2f32, height / 2f32)
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

    println!("{:?}", color);

    Ok(color)
}
