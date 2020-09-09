extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::genome::Genome;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::Neuron;
use self::neural_networks::neuron::NeuronTrait;
use self::neural_networks::neuron_activation::choose_random_activation_function::choose_random_activation_function;
use self::randomization::randomizer::Randomizer;

type GN = Genome<NeuralNetwork<Neuron>, Neuron>;

pub fn create_standard_training_population(
    number_of_neural_networks: u32,
    layers_definition: &[[usize; 2]],
    randomizer: &mut Randomizer,
) -> Result<Population<GN, NeuralNetwork<Neuron>, Neuron>, String> {
    Ok(Population::new_with_specified_layers(
        number_of_neural_networks,
        layers_definition,
        |genome_identifier, layers_definition, randomizer| {
            Genome::new(
                genome_identifier,
                NeuralNetwork::new_with_specified_layers(
                    layers_definition,
                    randomizer,
                    |number_of_inputs: u32, randomizer: &mut Randomizer| {
                        Neuron::new(
                            number_of_inputs,
                            choose_random_activation_function(randomizer),
                            randomizer,
                        )
                    },
                ),
            )
        },
        randomizer,
    )?)
}
