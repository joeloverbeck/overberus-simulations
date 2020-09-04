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
    _randomizer: &mut V,
) {
    let distance_from_top =
        (manhattan_distance(i, j, image_width / 2, 0) as f64) / (image_height as f64);
    let distance_from_top_left = (manhattan_distance(i, j, 0, 0) as f64) / (image_height as f64);
    let distance_from_left =
        (manhattan_distance(i, j, 0, image_height / 2) as f64) / (image_height as f64);
    let distance_from_center = (manhattan_distance(i, j, image_width / 2, image_height / 2) as f64)
        / (image_height as f64);
    let distance_from_right = (manhattan_distance(i, j, image_width - 1, image_height / 2) as f64)
        / (image_height as f64);
    let distance_from_bottom = (manhattan_distance(i, j, image_width / 2, image_height - 1) as f64)
        / (image_height as f64);
    let distance_from_bottom_right = (manhattan_distance(i, j, image_height - 1, image_width - 1)
        as f64)
        / (image_height as f64);

    assert!(distance_from_top <= 2.0, "{}", distance_from_top);
    assert!(distance_from_top_left <= 2.0, "{}", distance_from_top_left);
    assert!(distance_from_left <= 2.0, "{}", distance_from_left);
    assert!(distance_from_center <= 2.0, "{}", distance_from_center);
    assert!(distance_from_right <= 2.0, "{}", distance_from_right);
    assert!(distance_from_bottom <= 2.0, "{}", distance_from_bottom);
    assert!(
        distance_from_bottom_right <= 2.0,
        "{}",
        distance_from_bottom_right
    );

    // They come from 0.0 to 2.0.

    let outputs = neural_network
        .propagate(&[
            distance_from_top / 2.0,
            distance_from_top_left / 2.0,
            distance_from_left / 2.0,
            distance_from_center / 2.0,
            distance_from_right / 2.0,
            distance_from_bottom / 2.0,
            distance_from_bottom_right / 2.0,
        ])
        .unwrap();

    assert!(outputs.len() == 4);

    neural_network_outputs.extend(outputs);
}
