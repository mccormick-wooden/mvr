use std::path::PathBuf;

pub fn is_source_valid(file: &PathBuf) -> Result<&PathBuf, String> {
    match file.exists() {
        true => Ok(file),
        false => Err(format!("File doesn't exist: {:#?}", file.as_path())),
    }
}
