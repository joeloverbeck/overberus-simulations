extern crate cosmopolitan_collapse;
extern crate file_system;
extern crate gym;
extern crate neural_networks;
extern crate randomization;

use self::file_system::deserialize_json_from_string::deserialize_json_from_string;
use self::file_system::read_file_to_string::read_file_to_string;
use self::file_system::save_json::save_json;
use self::gym::domain::models::cosmopolitan_collapse::train_for_domain::train_for_domain;
use self::neural_networks::evolution::domain::genome::Genome;
use self::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::get_index_max_output::get_index_max_output;
use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::Neuron;
use self::randomization::randomizer::Randomizer;

fn main() {
    let winner = train_for_domain(
        &[[3, 4], [4, 4], [4, 3]],
        |generation_number: u32, current_winner: &Option<Genome<NeuralNetwork<Neuron>, Neuron>>| {
            if generation_number <= 10 || current_winner.as_ref().unwrap().get_fitness() < 20.0 {
                true
            } else {
                false
            }
        },
        |genomes: &mut Vec<Genome<NeuralNetwork<Neuron>, Neuron>>,
         _randomizer: &mut Randomizer|
         -> Result<(), String> {
            for genome in genomes.iter_mut() {
                // For this domain:
                // Inputs: [0] CavesPresent [1] BuildingsPresent [2] IsHomeless
                // Outputs: [0] SettleInCave [1] SettleInBuilding [2] SetUpCamp

                let outputs = genome
                    .get_neural_network()
                    .propagate(&[1.0, 1.0, 1.0])
                    .unwrap();

                if get_index_max_output(&outputs) == 1 {
                    let current_fitness = genome.get_fitness();
                    genome.set_fitness(current_fitness + 10.0);
                }

                let outputs = genome
                    .get_neural_network()
                    .propagate(&[0.0, 1.0, 1.0])
                    .unwrap();

                if get_index_max_output(&outputs) == 1 {
                    let current_fitness = genome.get_fitness();
                    genome.set_fitness(current_fitness + 10.0);
                }
            }

            Ok(())
        },
    )
    .unwrap();

    assert_eq!(winner.get_fitness(), 20.0);

    let filename =
        "data/cosmopolitan_collapse/neural_networks/low_level/settling_in_a_component/urban.json";

    println!("Saving neural network to {:?}", filename);

    save_json(filename, winner.get_neural_network()).unwrap();

    let neural_network = deserialize_json_from_string::<NeuralNetwork<Neuron>>(
        &read_file_to_string(filename).unwrap(),
    )
    .unwrap();

    println!("Running some tests with stored neural network...");

    let mut inputs = &[1.0, 1.0, 1.0];

    println!(
        "For inputs: {:?} -> outputs: {:?}",
        inputs,
        neural_network.propagate(inputs)
    );

    inputs = &[0.0, 1.0, 1.0];

    println!(
        "For inputs: {:?} -> outputs: {:?}",
        inputs,
        neural_network.propagate(inputs)
    );

    inputs = &[0.0, 0.0, 1.0];

    println!(
        "For inputs: {:?} -> outputs: {:?}",
        inputs,
        neural_network.propagate(inputs)
    );

    inputs = &[0.0, 0.0, 0.0];

    println!(
        "For inputs: {:?} -> outputs: {:?}",
        inputs,
        neural_network.propagate(inputs)
    );

    inputs = &[0.0, 1.0, 1.0];

    println!(
        "For inputs: {:?} -> outputs: {:?}",
        inputs,
        neural_network.propagate(inputs)
    );

    inputs = &[1.0, 0.0, 1.0];

    println!(
        "For inputs: {:?} -> outputs: {:?}",
        inputs,
        neural_network.propagate(inputs)
    );
}
