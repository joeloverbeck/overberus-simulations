use get_filenames_from_directory_that_end_with_extension::get_filenames_from_directory_that_end_with_extension;
use remove_file::remove_file;

pub fn remove_files_with_extension_in_directory(
    directory: &str,
    extension: &str,
) -> Result<(), String> {
    let stored_files = get_filenames_from_directory_that_end_with_extension(directory, extension);
    stored_files
        .iter()
        .for_each(|stored_file| remove_file(stored_file).unwrap());

    Ok(())
}
