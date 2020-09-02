use std::fs::create_dir_all;

use std::path::Path;

pub fn create_all_directories_on_path(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    if let Some(directories) = path.parent() {
        if let Err(error) = create_dir_all(directories) {
            panic!("Attempted to create all directories missing in path {:?}, but failed with the following error: {:?}", file_path, error);
        }
    }

    Ok(())
}
