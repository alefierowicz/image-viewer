use crate::{jpeg, ppm, volume::Volume};
use std::path::Path;
pub struct LoadedFile {
    pub name: String,
    pub volume: Volume,
}

pub fn load_file(path: &Path) -> Result<LoadedFile, String> {
    // starting with PPM
    let data = std::fs::read(path).map_err(|e| format!("could not read {}: {}", path.display(), e))?;
    let Some(os_file_name) = path.file_name() else { return Err("could not read file name.".into()) };
    let volume = if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        jpeg::parse(&data)
    } else if data.starts_with(b"P") {
        ppm::parse(&data)
    } else {
        return Err("unsupported file format".into());
    }?;
    // TEST
    // jpeg::save(&volume, Path::new("test.jpg"), 90).unwrap();
    Ok(LoadedFile { name: os_file_name.to_string_lossy().into(), volume })
}
