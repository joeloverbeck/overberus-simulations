use does_file_exist::does_file_exist;
use std::fs;

pub fn read_file_to_string(file_path: &str) -> Result<String, String> {
    // If there isn't any file in the path given, crash.
    if !does_file_exist(file_path)? {
        panic!(
            "Attempted to read a file to a string, but the file didn't exist! File path: {:?}",
            file_path
        );
    }

    match fs::read_to_string(file_path) {
        Ok(text) => Ok(text),
        Err(error) => panic!(
            "Failed to read file path {:?} to a string. Error: {:?} ",
            file_path, error
        ),
    }
}
