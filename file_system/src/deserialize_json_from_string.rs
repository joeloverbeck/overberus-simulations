extern crate serde;

use self::serde::Deserialize;

pub fn deserialize_json_from_string<'a, T: Deserialize<'a>>(
    file_as_string: &'a str,
) -> Result<T, String> {
    match serde_json::from_str(&file_as_string) {
        Err(error) => Err(format!(
            "Failed to deserialize json from string {:?}. Error: {:?}",
            file_as_string, error
        )),
        Ok(value) => Ok(value),
    }
}
