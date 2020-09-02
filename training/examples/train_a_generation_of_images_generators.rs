extern crate file_system;
extern crate gym;
extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
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

use self::file_system::does_file_exist::does_file_exist;

fn main() {
    let console_display_controller = ConsoleDisplayController::new();

    console_display_controller
        .write_announcement("Train a generation of images generators")
        .unwrap();

    console_display_controller.write_information("This program will attempt to load a saved population of images generators located in './data/training_population.json'. If there isn't one, the program will start a new training session.").unwrap();

    let mut training_population: Option<
        Population<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>,
    > = None;

    let mut randomizer = Randomizer::new();

    let layers_definition = &[[6, 10], [10, 15], [15, 4]];

    let number_of_neural_networks = 100;

    let training_population_path = "data/images_generation/training_population.json";

    if does_file_exist("data/images_generation/training_population.json").unwrap() {
        todo!();
    } else {
        console_display_controller.write_information("Couldn't find a previous training population. Starting a training session from scratch...").unwrap();

        if let Ok(population) = Population::new_with_specified_layers(
            number_of_neural_networks,
            layers_definition,
            |layers_definition, randomizer| {
                Genome::new(NeuralNetwork::new_with_specified_layers(
                    layers_definition,
                    randomizer,
                    |number_of_inputs, randomizer| {
                        Neuron::new(number_of_inputs, ActivationFunctions::Softplus, randomizer)
                    },
                ))
            },
            &mut randomizer,
        ) {
            training_population = Some(population);
        } else {
        }
    }

    let population = training_population.unwrap();

    let mut gym_controller = GymController::new(
        population,
        |generation_number| if generation_number >= 1 { false } else { true },
        |genomes, randomizer| {
            console_display_controller
                .write_information(
                    format!("Will create pngs from {:?} neural networks.", genomes.len()).as_str(),
                )
                .unwrap();

            for (index, genome) in genomes.iter().enumerate() {
                let dt = Local::now();

                let filename = format!(
                    "data/images_generation/genome_{}_{}{}{}_{}{}{}.png",
                    index + 1,
                    dt.year(),
                    dt.month(),
                    dt.day(),
                    dt.hour(),
                    dt.minute(),
                    dt.second()
                );

                save_json(
                    format!("data/images_generation/genome_{}.json", index + 1).as_str(),
                    genome.get_neural_network(),
                )?;

                console_display_controller
                    .write_information(
                        format!(
                            "Will write to file the output of genome {} as {}",
                            index + 1,
                            filename
                        )
                        .as_str(),
                    )
                    .unwrap();

                generate_png_from_neural_network(
                    80,
                    80,
                    genome.get_neural_network(),
                    filename.as_str(),
                    randomizer,
                )
                .unwrap();
            }

            Ok(())
        },
    );

    let new_population = gym_controller
        .train(
            |neural_network| Genome::new(neural_network),
            || NeuralNetwork::new(),
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Softplus, randomizer)
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
                "Next generation produced, with {}",
                new_population.get_size()
            )
            .as_str(),
        )
        .unwrap();

    save_json(training_population_path, &new_population).unwrap();

    console_display_controller
        .write_information(
            format!("Trained generation saved as {}", training_population_path).as_str(),
        )
        .unwrap();
}
