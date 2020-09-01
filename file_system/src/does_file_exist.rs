use std::path::Path;

pub fn does_file_exist(file_path: &str) -> Result<bool, String> {
    Ok(Path::new(file_path).exists())
}
