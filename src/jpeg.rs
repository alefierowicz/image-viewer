use crate::volume::Volume;
use image::ImageFormat::Jpeg;
use image::load_from_memory_with_format;
use std::path::Path;

pub fn parse(bytes: &[u8]) -> Result<Volume, String> {
    let result =
        load_from_memory_with_format(bytes, Jpeg).map_err(|e| format!("failed to load bytes as jpeg: {e}"))?.to_rgb8();
    // no need for robust error checking here imo since the func above does most of it for us, should map to a string properly
    let width = result.width();
    let height = result.height();

    let data: Vec<u16> = result.into_raw().iter().map(|b| *b as u16).collect();

    Volume::new(width as usize, height as usize, u8::MAX as u16, data)
}

pub fn save(volume: &Volume, path: &Path, quality: u8) -> Result<(), String> {
    let pixels = volume.render();
    let file = std::fs::File::create(path).map_err(|e| format!("failed to create export file: {e}"))?;

    let mut writer = std::io::BufWriter::new(file);
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, quality)
        .encode(&pixels, volume.width() as u32, volume.height() as u32, image::ExtendedColorType::Rgb8)
        .map_err(|e| format!("failed to encode jpeg file:{e}"))
}
