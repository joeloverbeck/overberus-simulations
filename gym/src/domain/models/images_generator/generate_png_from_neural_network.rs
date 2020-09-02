extern crate close_file;
extern crate file_system;
extern crate neural_networks;
extern crate randomization;

use domain::models::images_generator::generate_png_from_neural_network::file_system::does_file_exist::does_file_exist;
use domain::models::images_generator::generate_png_from_neural_network::file_system::create_all_directories_on_path::create_all_directories_on_path;
use domain::models::images_generator::generate_pixels_from_neural_network::generate_pixels_from_neural_network;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::RandomizerTrait;

extern crate png_encode_mini;
use self::png_encode_mini::write_rgba_from_u8;

use self::close_file::Closable;

pub fn generate_png_from_neural_network<
    T: NeuralNetworkTrait<U>,
    U: NeuronTrait,
    V: RandomizerTrait,
>(
    image_width: u32,
    image_height: u32,
    neural_network: &T,
    save_path: &str,
    randomizer: &mut V,
) -> Result<(), String> {
    // generate pixels from neural network
    let pixels =
        generate_pixels_from_neural_network(neural_network, image_width, image_height, randomizer)?;

    create_all_directories_on_path(save_path)?;

    if does_file_exist(save_path)? {
        panic!(
            "Was going to save image to {:?}, but there was already a file with that name in that path.",
            save_path
        );
    }

    let mut f = std::fs::File::create(save_path).unwrap();

    if let Err(error) =
        write_rgba_from_u8(&mut f, &pixels[..], image_width as u32, image_height as u32)
    {
        panic!(
            "An error happened while attempting to save a vector of pixels to a png file: {:?}",
            error
        );
    }

    if let Err(error) = f.close() {
        panic!(
            "An error occurred while trying to close a file handle for save path {:?}. Error: {:?}",
            save_path, error
        );
    }

    // Sanity check
    if !does_file_exist(save_path)? {
        panic!(
            "Although the image should have been saved to {:?}, I couldn't find it there!",
            save_path
        );
    }

    Ok(())
}
