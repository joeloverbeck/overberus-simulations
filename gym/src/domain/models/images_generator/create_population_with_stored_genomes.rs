extern crate file_system;
extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;

use self::neural_networks::evolution::domain::genome::Genome;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::evolution::domain::population::PopulationTrait;
use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neuron::Neuron;

use self::file_system::deserialize_json_from_string::deserialize_json_from_string;
use self::file_system::get_filenames_from_directory_that_end_with_extension::get_filenames_from_directory_that_end_with_extension;
use self::file_system::read_file_to_string::read_file_to_string;

type GN = Genome<NeuralNetwork<Neuron>, Neuron>;

pub fn create_population_with_stored_genomes(
    saved_genomes_directory: &str,
    console_display_controller: &ConsoleDisplayController,
) -> Result<Option<Population<GN, NeuralNetwork<Neuron>, Neuron>>, String> {
    console_display_controller.write_information("Found previous genomes saved in 'data/images_generation'. Will create a population with them.").unwrap();

    let stored_genome_filenames =
        get_filenames_from_directory_that_end_with_extension(saved_genomes_directory, "json");

    let mut population =
        Population::<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>::new();

    stored_genome_filenames.iter().for_each(|stored_genome| {
        let file_as_string = read_file_to_string(stored_genome).unwrap();

        let possible_genome =
            deserialize_json_from_string::<Genome<NeuralNetwork<Neuron>, Neuron>>(&file_as_string);

        match possible_genome {
            Ok(genome) => population.add(genome).unwrap(),
            Err(error) => console_display_controller
                .write_alert(
                    format!(
                        "Failed to load stored genome {} due to error: {}",
                        stored_genome, error
                    )
                    .as_str(),
                )
                .unwrap(),
        }
    });

    Ok(Some(population))
}
