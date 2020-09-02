pub fn normalize_to_rgba_range(value: f64) -> u8 {
    if value >= 1.0 {
        255
    } else if value <= 0.0 {
        0
    } else {
        let converted_float = value * 256.0;
        converted_float.floor() as u8
    }
}
