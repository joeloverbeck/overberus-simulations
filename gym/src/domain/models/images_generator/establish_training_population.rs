extern crate file_system;
extern crate neural_networks;
extern crate randomization;
extern crate serde;
extern crate user_interface;

use self::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::RandomizerTrait;
use self::serde::Deserialize;
use self::serde::Serialize;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use domain::models::images_generator::constants::SAVED_GENOMES_DIRECTORY;

use self::file_system::are_there_filenames_with_extension_in_directory::are_there_filenames_with_extension_in_directory;
use domain::models::images_generator::create_new_population::create_new_population;
use domain::models::images_generator::create_population_with_stored_genomes::create_population_with_stored_genomes;

pub fn establish_training_population<
    'a,
    T: GenomeTrait<U, V> + Clone + Serialize + Deserialize<'a>,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
    W: DisplayControllerTrait,
    X: RandomizerTrait,
    Y: Fn() -> Population<T, U, V>,
    Z: Fn(&mut X) -> Result<Population<T, U, V>, String>,
>(
    population_creator: Y,
    defined_population_creator: Z,
    display_controller: &W,
    saved_genomes_as_strings: &'a [String],
    randomizer: &mut X,
) -> Result<Option<Population<T, U, V>>, String> {
    if are_there_filenames_with_extension_in_directory(SAVED_GENOMES_DIRECTORY, "json").unwrap() {
        match create_population_with_stored_genomes(
            display_controller,
            population_creator,
            saved_genomes_as_strings,
        ) {
            Ok(population) => Ok(population),
            Err(error) => {
                display_controller
                    .write_alert(
                        format!(
                            "Failed while creating a population with the stored genomes. Error: {}",
                            error
                        )
                        .as_str(),
                    )
                    .unwrap();
                Err(format!(
                    "Failed to create a population with stores genomes. Error: {}",
                    error
                ))
            }
        }
    } else {
        display_controller.write_information("Couldn't find a previous training population. Starting a training session from scratch...").unwrap();

        match create_new_population(defined_population_creator, randomizer) {
            Ok(population) => Ok(population),
            Err(error) => {
                display_controller.write_alert(format!("Failed while creating a new population to start from scratch. Error: {}", error).as_str()).unwrap();
                Err(format!(
                    "Failed to create a new training population. Error: {}",
                    error
                ))
            }
        }
    }
}
