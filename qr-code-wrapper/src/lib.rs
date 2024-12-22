//! This is a library crate for wrapping [`qrcode-generator`](https://docs.rs/qrcode-generator/latest/qrcode_generator/) crate.

extern crate qrcode_generator;

use base64::{engine::general_purpose::STANDARD, Engine};
pub use qrcode_generator::*;

/// Encode text to base64 string of a PNG image in memory.
#[inline]
pub fn to_png_to_base64_str_from_str<S: AsRef<str>>(
    text: S,
    ecc: QrCodeEcc,
    size: usize,
) -> Result<String, QRCodeError> {
    let png = to_png_to_vec_from_str(text, ecc, size)?;
    let base64_encoded_str = STANDARD.encode(png);
    Ok(base64_encoded_str)
}
