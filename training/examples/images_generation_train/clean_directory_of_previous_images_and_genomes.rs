extern crate file_system;

use file_system::remove_files_with_extension_in_directory::remove_files_with_extension_in_directory;

pub fn clean_directory_of_previous_images_and_genomes(
    saved_genomes_directory: &str,
) -> Result<(), String> {
    remove_files_with_extension_in_directory(saved_genomes_directory, "png")?;
    remove_files_with_extension_in_directory(saved_genomes_directory, "json")?;

    Ok(())
}
