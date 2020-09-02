extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::Randomizer;
use neural_networks::evolution::controllers::create_next_generation::create_next_generation;
use neural_networks::evolution::domain::create_genome::create_genome;
use neural_networks::evolution::domain::genome::Genome;
use neural_networks::neural_network::NeuralNetwork;
use neural_networks::neuron::Neuron;
use neural_networks::neuron_activation::activation_functions::ActivationFunctions;

fn main() {
    let mut randomizer = Randomizer::new();

    let layers_definition = &[[4, 3], [3, 2], [2, 1]];

    if let Ok(population) =
        Population::new_with_specified_layers(10, layers_definition, create_genome, &mut randomizer)
    {
        println!("Created population: {:?}", population);

        let next_generation = create_next_generation(
            &population,
            Genome::new,
            NeuralNetwork::new,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
            &mut randomizer,
        );

        println!("**Next generation produced**");
        println!("{:?}", next_generation);
    } else {
        println!("Failed to create population.");
    }
}
