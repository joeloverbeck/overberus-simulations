extern crate file_system;
extern crate neural_networks;
extern crate user_interface;

use user_interface::controllers::console_input_controller::ConsoleInputController;
use user_interface::controllers::console_input_controller_trait::ConsoleInputControllerTrait;

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

/// Will get input arguments, deserialize that genome, assign the passed fitness to it, and reserialize it back.
fn main() {
    let console_display_controller = ConsoleDisplayController::new();
    let console_input_controller = ConsoleInputController::new();

    console_display_controller.write_information("This is program is intended to rate the fitness of a specific genome, with its identifier passed as an argument.").unwrap();

    if !console_input_controller.does_console_argument_exist(1) {
        console_display_controller.crash_with_alert(
            "You should pass the genome identifier (a digit) as the first argument.",
        );
    }

    if !console_input_controller.does_console_argument_exist(2) {
        console_display_controller.crash_with_alert(
            "You should pass the fitness score (a float) as the second argument.",
        );
    }

    let possible_genome_identifier =
        console_input_controller.parse_console_argument_number_as_type::<u32>(1);
    let possible_fitness_score =
        console_input_controller.parse_console_argument_number_as_type::<f64>(2);

    
    if let Err(error) = possible_genome_identifier.clone() {
        console_display_controller.crash_with_alert(format!("The first argument should be a valid genome identifier (a digit). You passed '{}'. Error: {}", console_input_controller.get_console_argument_number(1), error).as_str());
    }

    if let Err(error) = possible_fitness_score.clone() {
        console_display_controller.crash_with_alert(format!("The second argument should be a valid floating number, for the fitness score. You passed '{}'. Error: {}", console_input_controller.get_console_argument_number(2), error).as_str());
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
        Ok(mut genome) => {
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
    }
}
