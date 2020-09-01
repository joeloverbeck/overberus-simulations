extern crate serde;
extern crate serde_json;

use self::serde::Serialize;

use std::fs::{create_dir_all, File};

use std::path::Path;

pub fn save_json<T: Serialize>(file_path: &str, value_to_serialize: &T) -> Result<(), String> {
    let path = Path::new(file_path);

    if let Some(directories) = path.parent() {
        if let Err(error) = create_dir_all(directories) {
            panic!("Attempted to create all directories missing in path {:?}, but failed with the following error: {:?}", file_path, error);
        }
    }

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
