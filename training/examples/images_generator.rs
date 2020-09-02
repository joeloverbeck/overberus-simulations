extern crate chrono;
extern crate neural_networks;
extern crate png_encode_mini;
extern crate randomization;
use chrono::prelude::*;

use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::png_encode_mini::write_rgba_from_u8;
use self::randomization::randomizer::Randomizer;
use self::randomization::randomizer::RandomizerTrait;
use neural_networks::neuron::Neuron;
use neural_networks::neuron::NeuronTrait;

fn normalize_to_rgba_range(value: f64) -> u8 {
    if value >= 1.0 {
        255
    } else if value <= 0.0 {
        0
    } else {
        let converted_float = value * 256.0;
        converted_float.floor() as u8
    }
}

pub fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}

fn generate_image() {
    let mut randomizer = Randomizer::new();

    // image from bottom to top 3x2
    let image_width = 600;
    let image_height = 600;
    let mut image: Vec<u8> = Vec::new();

    let neural_network = NeuralNetwork::new_with_specified_layers(
        &[[4, 5], [5, 20], [20, 10], [10, 6], [6, 4]],
        &mut randomizer,
        |number_of_inputs, randomizer| Neuron::new(number_of_inputs, randomizer),
    );

    let mut neural_network_outputs: Vec<f64> = Vec::new();

    println!("Requesting output from neural network...");

    for i in 0..image_height {
        for j in 0..image_width {
            let distance_from_top_left = manhattan_distance(i, j, 0, 0);
            let distance_from_center = manhattan_distance(i, j, image_height / 2, image_width / 2);
            let distance_from_bottom_right =
                manhattan_distance(i, j, image_height - 1, image_width - 1);

            let outputs = neural_network
                .propagate(&[
                    (distance_from_center as f64),
                    (distance_from_top_left as f64),
                    (distance_from_bottom_right as f64),
                    randomizer.generate_float_from_0_to_1(),
                ])
                .unwrap();

            assert!(outputs.len() == 4);

            neural_network_outputs.extend(outputs);
        }
    }

    for index in 0..image_width * image_height * 4 {
        image.push(normalize_to_rgba_range(neural_network_outputs[index]));
    }

    println!(
        "Length of rgba image: {:?}. It should be {:?}",
        image.len(),
        image_width * image_height * 4
    );

    let dt = Local::now();

    let mut f = std::fs::File::create(format!(
        "test_{}{}{}_{}{}{}.png",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second()
    ))
    .unwrap();

    match write_rgba_from_u8(&mut f, &image[..], image_width as u32, image_height as u32) {
        Ok(_) => println!("Written image!"),
        Err(e) => println!("Error {:?}", e),
    }
}

fn main() {
    loop {
        generate_image();
    }
}
