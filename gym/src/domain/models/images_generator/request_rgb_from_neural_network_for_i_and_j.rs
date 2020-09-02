extern crate geometry;
extern crate neural_networks;
extern crate randomization;

use self::geometry::manhattan_distance::manhattan_distance;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::RandomizerTrait;

pub fn request_rgb_from_neural_network_for_i_and_j<
    T: NeuralNetworkTrait<U>,
    U: NeuronTrait,
    V: RandomizerTrait,
>(
    i: u32,
    j: u32,
    image_width: u32,
    image_height: u32,
    neural_network: &T,
    neural_network_outputs: &mut Vec<f64>,
    randomizer: &mut V,
) {
    let distance_from_top_left = manhattan_distance(i, j, 0, 0);
    let distance_from_left = manhattan_distance(i, j, image_height / 2, 0);
    let distance_from_center = manhattan_distance(i, j, image_height / 2, image_width / 2);
    let distance_from_right = manhattan_distance(i, j, 0, image_width / 2);
    let distance_from_bottom_right = manhattan_distance(i, j, image_height - 1, image_width - 1);

    let outputs = neural_network
        .propagate(&[
            (distance_from_top_left as f64) / (image_height as f64),
            (distance_from_left as f64) / (image_height as f64),
            (distance_from_center as f64) / (image_height as f64),
            (distance_from_right as f64) / (image_height as f64),
            (distance_from_bottom_right as f64) / (image_height as f64),
            randomizer.generate_float_from_0_to_1(),
        ])
        .unwrap();

    assert!(outputs.len() == 4);

    neural_network_outputs.extend(outputs);
}
