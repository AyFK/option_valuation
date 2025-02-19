
/// Takes an '#RRGGBB' `&str` and a `f64` value in the region
/// (0, 1), and reutrns a new string '#AARRGGBB' which accounts
/// for the 'alpha' value of the given color.
#[allow(dead_code)]
pub fn alpha_hex(hex: &str, alpha: f64) -> String {
    // ensure the input is '#RRGGBB'
    assert!(hex.starts_with('#') && hex.len() == 7, "Expected format: #RRGGBB.");

    // clamp alpha to [0, 1] and convert to 0-255
    let alpha = 1.0 - alpha.clamp(0.0, 1.0);
    let alpha_u8 = (alpha * 255.0).round() as u8;

    // construct the new string with alpha in front: "#AARRGGBB"
    return format!("#{:02X}{}", alpha_u8, &hex[1..]);
}
