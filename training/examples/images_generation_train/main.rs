pub mod clean_directory_of_previous_images_and_genomes;

extern crate file_system;
extern crate gym;
extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

use file_system::get_filenames_from_directory_that_end_with_extension::get_filenames_from_directory_that_end_with_extension;
use file_system::read_file_to_string::read_file_to_string;
use gym::domain::models::images_generator::constants::LAYERS_DEFINITION;
use gym::domain::models::images_generator::constants::NUMBER_OF_NEURAL_NETWORKS;
use gym::domain::models::images_generator::constants::SAVED_GENOMES_DIRECTORY;
use gym::domain::models::images_generator::establish_training_population::establish_training_population;
use gym::domain::models::images_generator::process_generation_of_images_from_neural_networks::process_generation_of_images_from_neural_networks;
use gym::domain::models::images_generator::save_evolved_population::save_evolved_population;
use std::process;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use file_system::create_all_directories_on_path::create_all_directories_on_path;
use gym::controllers::gym_controller::GymController;
use neural_networks::evolution::domain::genome::Genome;
use neural_networks::evolution::domain::population::Population;
use neural_networks::evolution::domain::population::PopulationTrait;
use neural_networks::neural_network::NeuralNetwork;
use neural_networks::neural_network::NeuralNetworkTrait;
use neural_networks::neuron::Neuron;
use neural_networks::neuron::NeuronTrait;
use neural_networks::neuron_activation::activation_functions::ActivationFunctions;
use randomization::randomizer::Randomizer;

fn main() {
    let console_display_controller = ConsoleDisplayController::new();

    console_display_controller
        .write_announcement("Train a generation of images generators")
        .unwrap();

    console_display_controller.write_information(format!("This program will attempt to load a saved population of images generators (genomes) located in '{}'. Otherwise, the program will start a new training session.", SAVED_GENOMES_DIRECTORY).as_str()).unwrap();

    let mut randomizer = Randomizer::new();

    console_display_controller
        .write_section("Evolution of a new generation")
        .unwrap();

    create_all_directories_on_path(SAVED_GENOMES_DIRECTORY).unwrap();

    let saved_genomes_as_strings: Vec<String> = {
        get_filenames_from_directory_that_end_with_extension(SAVED_GENOMES_DIRECTORY, "json")
            .iter()
            .map(|stored_genome| read_file_to_string(stored_genome).unwrap())
            .collect()
    };

    let population = {
        match establish_training_population(
            || Population::new(),
            |randomizer| {
                Population::new_with_specified_layers(
                    NUMBER_OF_NEURAL_NETWORKS,
                    LAYERS_DEFINITION,
                    |genome_identifier, layers_definition, randomizer| {
                        Genome::new(
                            genome_identifier,
                            NeuralNetwork::new_with_specified_layers(
                                layers_definition,
                                randomizer,
                                |number_of_inputs, randomizer| {
                                    Neuron::new(
                                        number_of_inputs,
                                        ActivationFunctions::Swish,
                                        randomizer,
                                    )
                                },
                            ),
                        )
                    },
                    randomizer,
                )
            },
            &console_display_controller,
            &saved_genomes_as_strings,
            &mut randomizer,
        ) {
            Ok(possible_population) => possible_population.unwrap(),
            Err(error) => {
                console_display_controller
                    .write_alert(
                        format!(
                            "Failed to establish a training population. Error: {}",
                            error
                        )
                        .as_str(),
                    )
                    .unwrap();
                process::exit(0);
            }
        }
    };

    // If at this point there are no genomes in the population, leave.
    if population.get_size() == 0 {
        console_display_controller
            .write_alert("Couldn't create population of genomes. The program will exit.")
            .unwrap();
        process::exit(0);
    } else if population.get_size() == 1 {
        console_display_controller.write_alert("The population of genomes is insufficient for evolution. Please remove the stored genomes and start again.").unwrap();
        process::exit(0);
    }

    // Clean the directory of every previous png and saved genome.
    if let Err(error) = clean_directory_of_previous_images_and_genomes::clean_directory_of_previous_images_and_genomes(SAVED_GENOMES_DIRECTORY) {
        console_display_controller.write_alert(format!("Failed while cleaning the working directory of every previous png and saved genome. Error: {}", error).as_str()).unwrap();
        process::exit(0);
    }

    let mut gym_controller = GymController::new(
        population,
        |generation_number| if generation_number >= 1 { false } else { true },
        |genomes, randomizer| {
            process_generation_of_images_from_neural_networks(
                genomes,
                randomizer,
                &console_display_controller,
            )
        },
        |evolved_population| save_evolved_population(evolved_population),
    );

    let new_population = gym_controller
        .train(
            |genome_identifier, neural_network| Genome::new(genome_identifier, neural_network),
            || NeuralNetwork::new(),
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Swish, randomizer)
            },
            |_generation_number, _population| {
                console_display_controller
                    .write_information("Finished.")
                    .unwrap()
            },
            &mut randomizer,
        )
        .unwrap();

    console_display_controller
        .write_announcement(
            format!(
                "Next generation produced, with {} genomes",
                new_population.get_size()
            )
            .as_str(),
        )
        .unwrap();

    console_display_controller
        .write_instruction("Look over the generated images, and of those you like you should open their corresponding json file and edit the fitness (at the end of the file) to a higher float than 0.0.")
        .unwrap();

    console_display_controller
        .write_information("The program has finished successfully.")
        .unwrap();
}
