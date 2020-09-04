extern crate geometry;
extern crate neural_networks;
extern crate randomization;

use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::RandomizerTrait;
use domain::models::images_generator::normalize_to_rgba_range::normalize_to_rgba_range;
use domain::models::images_generator::request_rgb_from_neural_network_for_i_and_j::request_rgb_from_neural_network_for_i_and_j;

pub fn generate_pixels_from_neural_network<
    T: NeuralNetworkTrait<U>,
    U: NeuronTrait,
    V: RandomizerTrait,
>(
    neural_network: &T,
    image_width: u32,
    image_height: u32,
    randomizer: &mut V,
) -> Result<Vec<u8>, String> {
    let mut image: Vec<u8> = Vec::new();

    let mut neural_network_outputs: Vec<f64> = Vec::new();

    for i in 0..image_height {
        for j in 0..image_width {
            request_rgb_from_neural_network_for_i_and_j(
                i,
                j,
                image_width,
                image_height,
                neural_network,
                &mut neural_network_outputs,
                randomizer,
            );
        }
    }

    for index in 0..image_width * image_height * 4 {
        image.push(normalize_to_rgba_range(neural_network_outputs[index as usize].abs()));
    }

    Ok(image)
}
