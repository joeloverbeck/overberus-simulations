extern crate randomization;

use self::randomization::randomizer::Randomizer;
use evolution::domain::genome::Genome;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use neuron::Neuron;
use neuron::NeuronTrait;

pub fn create_genome(randomizer: &mut Randomizer) -> Genome<NeuralNetwork<Neuron>, Neuron> {
    Genome::new(NeuralNetwork::new_with_specified_layers(
        &[[4, 3], [3, 2], [2, 1]],
        randomizer,
        |number_of_inputs, randomizer| Neuron::new(number_of_inputs, randomizer),
    ))
}
