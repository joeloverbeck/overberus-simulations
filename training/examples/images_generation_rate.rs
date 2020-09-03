extern crate file_system;
extern crate neural_networks;
extern crate user_interface;

use std::process;

use self::user_interface::controllers::console_display_controller::ConsoleDisplayController;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;
use file_system::deserialize_json_from_string::deserialize_json_from_string;

use file_system::does_file_exist::does_file_exist;
use file_system::read_file_to_string::read_file_to_string;
use file_system::save_json::save_json;
use neural_networks::evolution::domain::genome::Genome;
use neural_networks::evolution::domain::genome::GenomeTrait;
use neural_networks::neural_network::NeuralNetwork;
use neural_networks::neuron::Neuron;

fn main() {
    let console_display_controller = ConsoleDisplayController::new();

    // Will get input arguments, deserialize that genome, assign the passed fitness to it, and reserialize it back.
    let possible_first_argument = std::env::args().nth(1);

    if possible_first_argument.is_none() {
        console_display_controller
            .write_alert("You should pass the genome identifier (a digit) as the first argument.")
            .unwrap();
        process::exit(0);
    }

    let possible_second_argument = std::env::args().nth(2);

    if possible_second_argument.is_none() {
        console_display_controller
            .write_alert("You should pass the fitness score (a float) as the second argument.")
            .unwrap();
        process::exit(0);
    }

    let first_argument = possible_first_argument.unwrap();
    let second_argument = possible_second_argument.unwrap();

    let possible_genome_identifier: Result<u32, std::num::ParseIntError> = first_argument.parse();
    let possible_fitness_score: Result<f64, std::num::ParseFloatError> = second_argument.parse();

    // Build the filename of the intended genome.
    if let Err(error) = possible_genome_identifier {
        console_display_controller
            .write_alert(format!("The first argument should be a valid genome identifier (a digit). You passed '{}'. Error: {}", first_argument, error).as_str())
            .unwrap();
        process::exit(0);
    }

    if let Err(error) = possible_fitness_score {
        console_display_controller
        .write_alert(format!("The second argument should be a valid floating number, for the fitness score. You passed '{}'. Error: {}", second_argument, error).as_str())
        .unwrap();
        process::exit(0);
    }

    let genome_identifier = possible_genome_identifier.unwrap();

    let genome_filename = format!("data/images_generation/genome_{}.json", genome_identifier);

    if !does_file_exist(&genome_filename).unwrap() {
        console_display_controller
            .write_alert(
                format!(
                    "There wasn't a file in the path {}. Likely no such genome was created.",
                    genome_filename
                )
                .as_str(),
            )
            .unwrap();
        process::exit(0);
    }

    let file_as_string = read_file_to_string(genome_filename.as_str()).unwrap();

    let possible_genome =
        deserialize_json_from_string::<Genome<NeuralNetwork<Neuron>, Neuron>>(&file_as_string);

    if let Err(error) = possible_genome {
        console_display_controller
            .write_alert(
                format!(
                    "Couldn't load the intended genome {} due to the following error: {}",
                    genome_identifier, error
                )
                .as_str(),
            )
            .unwrap();
        process::exit(0);
    }

    let mut genome = possible_genome.unwrap();

    assert_eq!(genome.get_identifier(), genome_identifier);

    let fitness_score = possible_fitness_score.unwrap();

    genome.set_fitness(fitness_score);

    save_json(genome_filename.as_str(), &genome).unwrap();

    console_display_controller
        .write_information(
            format!(
                "Genome {} now has a fitness of {}",
                genome.get_identifier(),
                fitness_score
            )
            .as_str(),
        )
        .unwrap();
}
