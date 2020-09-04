extern crate neural_networks;
extern crate randomization;

use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::Neuron;
use self::neural_networks::neuron::NeuronTrait;
use self::neural_networks::neuron_activation::activation_functions::ActivationFunctions;
use self::randomization::randomizer::Randomizer;

fn main() {
    let mut randomizer = Randomizer::new();

    let neural_network = NeuralNetwork::new_with_specified_layers(
        &[[7, 6], [6, 5], [5, 4], [4, 4]],
        &mut randomizer,
        |number_of_inputs, randomizer| {
            Neuron::new(number_of_inputs, ActivationFunctions::Relu, randomizer)
        },
    );

    let outputs = neural_network
        .propagate(&[0.5, 0.2, 0.7, 0.3, 0.6, 0.21, 0.67])
        .unwrap();

    println!("Outputs for Relu neurons: {:?}", outputs);
}
