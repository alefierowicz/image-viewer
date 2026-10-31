use crate::{ppm, volume::Volume};
use std::path::Path;
pub struct LoadedFile {
    pub name: String,
    pub volume: Volume,
}

pub fn load_file(path: &Path) -> Result<LoadedFile, String> {
    // starting with PPM
    let data = std::fs::read(path).map_err(|e| format!("could not read {}: {}", path.display(), e))?;
    let Some(os_file_name) = path.file_name() else { return Err("could not read file name.".into()) };
    let volume = ppm::parse(&data)?;

    Ok(LoadedFile { name: os_file_name.to_string_lossy().into(), volume })
}
