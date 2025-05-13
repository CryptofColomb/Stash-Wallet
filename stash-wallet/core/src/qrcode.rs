use qrcode::QrCode;
use image::Luma;
use std::io::Cursor;
use base64::{engine::general_purpose, Engine as _};

pub fn generate_qr_code(data: &str) -> Result<Vec<u8>, String> {
    let code = QrCode::new(data.as_bytes()).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    let mut buffer = Vec::new();

    image.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}

pub fn decode_qr_data(encoded: &str) -> Result<String, String> {
    let decoded = general_purpose::STANDARD.decode(encoded).map_err(|e| e.to_string())?;
    String::from_utf8(decoded).map_err(|e| e.to_string())
}