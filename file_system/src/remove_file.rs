use does_file_exist::does_file_exist;
use std::fs;

pub fn remove_file(file_path: &str) -> Result<(), String> {
    // If the file doesn't exist, there's no need to remove it.
    if !does_file_exist(file_path)? {
        return Ok(());
    }

    if let Err(error) = fs::remove_file(file_path) {
        panic!(
            "Wasn't able to remove file {:?} due to error: {:?}",
            file_path, error
        );
    }

    Ok(())
}
