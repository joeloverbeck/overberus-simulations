extern crate file_system;
extern crate gym;
extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

use std::process;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use file_system::are_there_filenames_with_extension_in_directory::are_there_filenames_with_extension_in_directory;
use file_system::create_all_directories_on_path::create_all_directories_on_path;
use file_system::deserialize_json_from_string::deserialize_json_from_string;
use file_system::get_filenames_from_directory_that_end_with_extension::get_filenames_from_directory_that_end_with_extension;
use file_system::read_file_to_string::read_file_to_string;
use file_system::remove_file::remove_file;
use file_system::save_json::save_json;
use gym::controllers::gym_controller::GymController;
use gym::domain::models::images_generator::generate_png_from_neural_network::generate_png_from_neural_network;
use neural_networks::evolution::domain::genome::Genome;
use neural_networks::evolution::domain::genome::GenomeTrait;
use neural_networks::evolution::domain::population::Population;
use neural_networks::evolution::domain::population::PopulationTrait;
use neural_networks::neural_network::NeuralNetwork;
use neural_networks::neural_network::NeuralNetworkTrait;
use neural_networks::neuron::Neuron;
use neural_networks::neuron::NeuronTrait;
use neural_networks::neuron_activation::activation_functions::ActivationFunctions;
use randomization::randomizer::Randomizer;

extern crate chrono;
use self::chrono::prelude::*;

fn main() {
    let console_display_controller = ConsoleDisplayController::new();

    console_display_controller
        .write_announcement("Train a generation of images generators")
        .unwrap();

    let saved_genomes_directory = "./data/images_generation/";

    console_display_controller.write_information(format!("This program will attempt to load a saved population of images generators (genomes) located in '{}'. Otherwise, the program will start a new training session.", saved_genomes_directory).as_str()).unwrap();

    let mut training_population: Option<
        Population<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>,
    > = None;

    let mut randomizer = Randomizer::new();

    let layers_definition = &[[7, 40], [40, 50], [50, 40], [40, 4]];

    let number_of_neural_networks = 20;

    console_display_controller
        .write_section("Evolution of a new generation")
        .unwrap();

    create_all_directories_on_path(saved_genomes_directory).unwrap();

    if are_there_filenames_with_extension_in_directory(saved_genomes_directory, "json").unwrap() {
        console_display_controller.write_information("Found previous genomes saved in 'data/images_generation'. Will create a population with them.").unwrap();

        let stored_genome_filenames =
            get_filenames_from_directory_that_end_with_extension(saved_genomes_directory, "json");

        let mut population = Population::<
            Genome<NeuralNetwork<Neuron>, Neuron>,
            NeuralNetwork<Neuron>,
            Neuron,
        >::new();

        stored_genome_filenames.iter().for_each(|stored_genome| {
            let file_as_string = read_file_to_string(stored_genome).unwrap();

            let possible_genome = deserialize_json_from_string::<
                Genome<NeuralNetwork<Neuron>, Neuron>,
            >(&file_as_string);

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

        training_population = Some(population);
    } else {
        console_display_controller.write_information("Couldn't find a previous training population. Starting a training session from scratch...").unwrap();

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
            &mut randomizer,
        ) {
            training_population = Some(population);
        } else {
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
    let stored_images =
        get_filenames_from_directory_that_end_with_extension(saved_genomes_directory, "png");
    stored_images
        .iter()
        .for_each(|stored_image| remove_file(stored_image).unwrap());

    let stored_genomes =
        get_filenames_from_directory_that_end_with_extension(saved_genomes_directory, "json");
    stored_genomes
        .iter()
        .for_each(|stored_genome| remove_file(stored_genome).unwrap());

    let mut gym_controller = GymController::new(
        population,
        |generation_number| if generation_number >= 1 { false } else { true },
        |genomes, randomizer| {
            console_display_controller
                .write_information(
                    format!("Will create pngs from {:?} neural networks.", genomes.len()).as_str(),
                )
                .unwrap();

            for genome in genomes.iter() {
                let dt = Local::now();

                let filename = format!(
                    "data/images_generation/genome_{}_{}{}{}_{}{}{}.png",
                    genome.get_identifier(),
                    dt.year(),
                    dt.month(),
                    dt.day(),
                    dt.hour(),
                    dt.minute(),
                    dt.second()
                );

                console_display_controller
                    .write_information(
                        format!(
                            "Will write to file the output of genome {} as {}",
                            genome.get_identifier(),
                            filename
                        )
                        .as_str(),
                    )
                    .unwrap();

                let image_dimension = 256;

                generate_png_from_neural_network(
                    image_dimension,
                    image_dimension,
                    genome.get_neural_network(),
                    filename.as_str(),
                    randomizer,
                )
                .unwrap();
            }

            Ok(())
        },
        |evolved_population| {
            let mut genome_identifiers: Vec<u32> = Vec::new();

            evolved_population
                .get_genomes()
                .unwrap()
                .iter()
                .for_each(|evolved_genome| {
                    if genome_identifiers.iter().any(|identifier| identifier == &evolved_genome.get_identifier()){
                        panic!("There were repeated identifiers for the genomes in the population. That should never happen.");
                    }else{
                        genome_identifiers.push(evolved_genome.get_identifier());
                    }
                    save_json(
                        format!(
                            "data/images_generation/genome_{}.json",
                            evolved_genome.get_identifier()
                        )
                        .as_str(),
                        evolved_genome,
                    )
                    .unwrap()
                });

            Ok(())
        },
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
