use std::fs;

use crate::{Watermark, constant::WATERMARK_FILE};

pub fn read_watermark() -> Result<Watermark, std::io::Error> {
    let data = fs::read_to_string(WATERMARK_FILE)?;
    let watermark: Watermark = serde_json::from_str(&data)?;
    Ok(watermark)
}

pub fn write_watermark(watermark: &Watermark) -> Result<(), std::io::Error> {
    let data = serde_json::to_string_pretty(watermark)?;
    fs::write(WATERMARK_FILE, data)?;
    Ok(())
}
