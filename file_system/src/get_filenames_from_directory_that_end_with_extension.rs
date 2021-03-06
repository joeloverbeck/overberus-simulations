use std::fs;

pub fn get_filenames_from_directory_that_end_with_extension(
    path: &str,
    extension: &str,
) -> Vec<String> {
    let files = fs::read_dir(path).unwrap();

    files
        .filter_map(Result::ok)
        .filter(|f| {
            if f.path().extension().is_some() {
                return f.path().extension().unwrap().to_os_string() == extension;
            }

            false
        })
        .map(|f| f.path().to_str().unwrap().to_string())
        .collect()
}
