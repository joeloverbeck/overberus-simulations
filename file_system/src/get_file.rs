use std::fs::File;
use std::path::Path;

pub fn get_file(file_path: &str) -> Result<std::fs::File, String> {
    let json_file_path = Path::new(file_path);
    Ok(File::open(json_file_path)
        .unwrap_or_else(|_| panic!("Didn't find a file in path {:?}", file_path)))
}
