extern crate serde;
extern crate serde_json;

use self::serde::Serialize;
use create_all_directories_on_path::create_all_directories_on_path;

use std::fs::File;

pub fn save_json<T: Serialize>(file_path: &str, value_to_serialize: &T) -> Result<(), String> {
    create_all_directories_on_path(file_path)?;

    match &File::create(file_path) {
        Ok(file) => {
            if let Err(error) = self::serde_json::to_writer(file, value_to_serialize) {
                panic!(
                    "Failed to save serializable value to path {:?} due to error: {:?} ",
                    file_path, error
                );
            }
        }
        Err(error) => panic!(
            "Wasn't able to create a file in the path {:?} due to the next error: {:?}",
            file_path, error
        ),
    }

    Ok(())
}
