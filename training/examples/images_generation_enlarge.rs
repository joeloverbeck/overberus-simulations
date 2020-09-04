extern crate file_system;
extern crate gym;
extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

use self::randomization::randomizer::Randomizer;

use user_interface::controllers::console_input_controller::ConsoleInputController;
use user_interface::controllers::console_input_controller_trait::ConsoleInputControllerTrait;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use file_system::deserialize_json_from_string::deserialize_json_from_string;

use file_system::does_file_exist::does_file_exist;
use file_system::read_file_to_string::read_file_to_string;
use neural_networks::evolution::domain::genome::Genome;
use neural_networks::evolution::domain::genome::GenomeTrait;
use neural_networks::neural_network::NeuralNetwork;
use neural_networks::neuron::Neuron;

use self::gym::domain::models::images_generator::generate_png_from_neural_network::generate_png_from_neural_network;
use self::gym::domain::models::images_generator::generate_time_tag_as_string::generate_time_tag_as_string;

use std::env;

fn main() {
    let console_display_controller = ConsoleDisplayController::new();
    let console_input_controller = ConsoleInputController::new();

    let image_dimension = 1440;

    console_display_controller.write_information(format!("This program is intended to produce {}x{} images from the genome identifiers passed as arguments.", image_dimension, image_dimension).as_str()).unwrap();

    if !console_input_controller.does_console_argument_exist(1) {
        console_display_controller.crash_with_alert(
            "You should pass at least one genome identifier (a digit) as an argument.",
        );
    }

    let mut randomizer = Randomizer::new();

    console_display_controller
        .write_section("Rendering enlarged images")
        .unwrap();

    for argument in env::args().skip(1) {
        // We should have a genome identifier.
        let possible_genome_identifier = argument.parse::<u32>();

        if let Err(error) = possible_genome_identifier.clone() {
            console_display_controller.crash_with_alert(format!("The first argument should be a valid genome identifier (a digit). You passed '{}'. Error: {}", console_input_controller.get_console_argument_number(1), error).as_str());
        }

        let genome_identifier = possible_genome_identifier.unwrap();

        // Build the filename of the intended genome.
        let genome_filename = format!("data/images_generation/genome_{}.json", genome_identifier);

        if !does_file_exist(&genome_filename).unwrap() {
            console_display_controller.crash_with_alert(
                format!(
                    "There wasn't a file in the path {}. Likely no such genome was created.",
                    genome_filename
                )
                .as_str(),
            );
        }

        let file_as_string = read_file_to_string(genome_filename.as_str()).unwrap();

        let possible_genome =
            deserialize_json_from_string::<Genome<NeuralNetwork<Neuron>, Neuron>>(&file_as_string);

        match possible_genome {
            Err(error) => console_display_controller.crash_with_alert(
                format!(
                    "Couldn't load the intended genome {} due to the following error: {}",
                    genome_identifier, error
                )
                .as_str(),
            ),
            Ok(genome) => {
                let genome_filename = format!(
                    "data/images_generation/enlarged_images/genome_{}_{}.png",
                    genome_identifier,
                    generate_time_tag_as_string()
                );

                console_display_controller
                    .write_information(
                        format!(
                            "Will create the enlarged image for genome {} in '{}'",
                            genome.get_identifier(),
                            genome_filename
                        )
                        .as_str(),
                    )
                    .unwrap();

                console_display_controller
                    .write_instruction("Rendering image...")
                    .unwrap();

                generate_png_from_neural_network(
                    image_dimension,
                    image_dimension,
                    genome.get_neural_network(),
                    genome_filename.as_str(),
                    &mut randomizer,
                )
                .unwrap();

                console_display_controller
                    .write_information(
                        format!(
                            "Enlarged image rendered for genome {}.",
                            genome.get_identifier()
                        )
                        .as_str(),
                    )
                    .unwrap();
            }
        }
    }
}
