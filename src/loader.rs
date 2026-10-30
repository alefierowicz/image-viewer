use crate::{ppm, volume::Volume};
use std::path::Path;
pub struct LoadedFile {
    pub name: String,
    pub volume: Volume,
}

pub fn load_file(path: &Path) -> Result<LoadedFile, String> {
    // starting with PPM
    let Ok(data) = std::fs::read(path) else { return Err(format!("could not read file: {}", path.to_string_lossy())) };
    let Some(os_file_name) = path.file_name() else { return Err("could not read file name.".into()) };
    let volume = ppm::parse(&data)?;

    return Ok(LoadedFile { name: os_file_name.to_string_lossy().into(), volume });
}
