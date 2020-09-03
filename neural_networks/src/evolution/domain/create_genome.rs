extern crate randomization;

use self::randomization::randomizer::Randomizer;
use evolution::domain::genome::Genome;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use neuron::Neuron;
use neuron::NeuronTrait;
use neuron_activation::activation_functions::ActivationFunctions;

pub fn create_genome(
    genome_identifier: u32,
    layers_definition: &[[usize; 2]],
    randomizer: &mut Randomizer,
) -> Genome<NeuralNetwork<Neuron>, Neuron> {
    Genome::new(
        genome_identifier,
        NeuralNetwork::new_with_specified_layers(
            layers_definition,
            randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        ),
    )
}
