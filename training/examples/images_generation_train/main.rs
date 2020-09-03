pub mod clean_directory_of_previous_images_and_genomes;

extern crate file_system;
extern crate gym;
extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

use gym::domain::models::images_generator::create_new_population::create_new_population;
use gym::domain::models::images_generator::process_generation_of_images_from_neural_networks::process_generation_of_images_from_neural_networks;
use gym::domain::models::images_generator::save_evolved_population::save_evolved_population;
use std::process;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use file_system::are_there_filenames_with_extension_in_directory::are_there_filenames_with_extension_in_directory;
use file_system::create_all_directories_on_path::create_all_directories_on_path;
use gym::controllers::gym_controller::GymController;
use gym::domain::models::images_generator::create_population_with_stored_genomes::create_population_with_stored_genomes;
use neural_networks::evolution::domain::genome::Genome;
use neural_networks::evolution::domain::population::Population;
use neural_networks::evolution::domain::population::PopulationTrait;
use neural_networks::neural_network::NeuralNetwork;
use neural_networks::neuron::Neuron;
use neural_networks::neuron::NeuronTrait;
use neural_networks::neuron_activation::activation_functions::ActivationFunctions;
use randomization::randomizer::Randomizer;

fn main() {
    let console_display_controller = ConsoleDisplayController::new();

    console_display_controller
        .write_announcement("Train a generation of images generators")
        .unwrap();

    let saved_genomes_directory = "./data/images_generation/";

    console_display_controller.write_information(format!("This program will attempt to load a saved population of images generators (genomes) located in '{}'. Otherwise, the program will start a new training session.", saved_genomes_directory).as_str()).unwrap();

    let training_population: Option<
        Population<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>,
    >;

    let mut randomizer = Randomizer::new();

    console_display_controller
        .write_section("Evolution of a new generation")
        .unwrap();

    create_all_directories_on_path(saved_genomes_directory).unwrap();

    if are_there_filenames_with_extension_in_directory(saved_genomes_directory, "json").unwrap() {
        match create_population_with_stored_genomes(
            saved_genomes_directory,
            &console_display_controller,
        ) {
            Ok(population) => training_population = population,
            Err(error) => {
                console_display_controller
                    .write_alert(
                        format!(
                            "Failed while creating a population with the stored genomes. Error: {}",
                            error
                        )
                        .as_str(),
                    )
                    .unwrap();
                process::exit(0);
            }
        }
    } else {
        console_display_controller.write_information("Couldn't find a previous training population. Starting a training session from scratch...").unwrap();

        match create_new_population(&mut randomizer) {
            Ok(population) => training_population = population,
            Err(error) => {
                console_display_controller.write_alert(format!("Failed while creating a new population to start from scratch. Error: {}", error).as_str()).unwrap();
                process::exit(0);
            }
        }
    }

    let population = training_population.unwrap();

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
    if let Err(error) = clean_directory_of_previous_images_and_genomes::clean_directory_of_previous_images_and_genomes(saved_genomes_directory) {
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
