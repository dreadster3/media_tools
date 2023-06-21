pub fn from_str_to_hex(rgba_string: &String) -> Result<String, std::num::ParseIntError> {
    let rgba_values: Vec<u8> = rgba_string
        .trim_matches(|p| p == '(' || p == ')')
        .split(',')
        .map(|s| s.trim().parse().unwrap_or_default())
        .collect();

    let mut color = String::from("#");

    for value in rgba_values {
        color.push_str(&format!("{:02X}", value));
    }

    return Ok(color);
}
