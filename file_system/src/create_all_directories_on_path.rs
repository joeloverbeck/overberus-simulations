use std::fs::create_dir_all;

use std::path::Path;

pub fn create_all_directories_on_path(file_path: &str) -> Result<(), String> {
    // First will need to remove filename from the path, if any.
    let mut path = Path::new(file_path);

    if path.extension().is_some() {
        path = path.parent().unwrap();
    }

    path.ancestors().for_each(|directories| {
        if let Err(error) = create_dir_all(directories) {
            panic!("Attempted to create all directories missing in path {:?}, but failed with the following error: {:?}", file_path, error);
        }
    });

    Ok(())
}
