extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::genome::Genome;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::Neuron;
use self::neural_networks::neuron::NeuronTrait;
use self::neural_networks::neuron_activation::activation_functions::ActivationFunctions;
use self::randomization::randomizer::RandomizerTrait;

type GN = Genome<NeuralNetwork<Neuron>, Neuron>;

pub fn create_new_population<T: RandomizerTrait>(
    randomizer: &mut T,
) -> Result<Option<Population<GN, NeuralNetwork<Neuron>, Neuron>>, String> {
    let number_of_neural_networks = 100;

    let layers_definition = &[[7, 40], [40, 50], [50, 40], [40, 4]];

    if let Ok(population) = Population::new_with_specified_layers(
        number_of_neural_networks,
        layers_definition,
        |genome_identifier, layers_definition, randomizer| {
            Genome::new(
                genome_identifier,
                NeuralNetwork::new_with_specified_layers(
                    layers_definition,
                    randomizer,
                    |number_of_inputs, randomizer| {
                        Neuron::new(number_of_inputs, ActivationFunctions::Swish, randomizer)
                    },
                ),
            )
        },
        randomizer,
    ) {
        return Ok(Some(population));
    }

    Err("Failed to create a new population for the images generation program.".to_string())
}
