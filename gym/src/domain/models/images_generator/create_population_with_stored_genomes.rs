extern crate file_system;
extern crate neural_networks;
extern crate randomization;
extern crate serde;
extern crate user_interface;

use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;

use self::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::evolution::domain::population::PopulationTrait;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;

use self::file_system::deserialize_json_from_string::deserialize_json_from_string;
use self::serde::Deserialize;

pub fn create_population_with_stored_genomes<
    'a,
    T: GenomeTrait<U, V> + Clone + Deserialize<'a>,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
    W: DisplayControllerTrait,
    X: Fn() -> Population<T, U, V>,
>(
    display_controller: &W,
    population_creator: X,
    saved_genomes_as_strings: &'a [String],
) -> Result<Option<Population<T, U, V>>, String> {
    display_controller.write_information("Found previous genomes saved in 'data/images_generation'. Will create a population with them.").unwrap();

    let mut population = population_creator();

    for saved_genome_as_string in saved_genomes_as_strings.iter() {
        let possible_genome = deserialize_json_from_string::<T>(&saved_genome_as_string);

        match possible_genome {
            Ok(genome) => population.add(genome).unwrap(),
            Err(error) => display_controller
                .write_alert(
                    format!(
                        "Failed to load stored genome {} due to error: {}",
                        saved_genome_as_string, error
                    )
                    .as_str(),
                )
                .unwrap(),
        }
    }

    Ok(Some(population))
}
