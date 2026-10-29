use crate::volume::Volume;
use std::path::Path;

pub struct LoadedFile {
    pub name: String,
    pub volume: Volume,
}

pub fn load_file(_path: &Path) -> Result<LoadedFile, String> {
    Err("not implemented yet".into())
}
