use get_filenames_from_directory_that_end_with_extension::get_filenames_from_directory_that_end_with_extension;

pub fn are_there_filenames_with_extension_in_directory(
    path: &str,
    extension: &str,
) -> Result<bool, String> {
    Ok(!get_filenames_from_directory_that_end_with_extension(path, extension).is_empty())
}
