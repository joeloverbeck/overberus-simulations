extern crate neural_networks;
extern crate randomization;
extern crate user_interface;

extern crate chrono;
use self::chrono::prelude::*;
use domain::models::images_generator::generate_png_from_neural_network::generate_png_from_neural_network;

use self::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::RandomizerTrait;
use self::user_interface::controllers::display_controller_trait::DisplayControllerTrait;

pub fn process_generation_of_images_from_neural_networks<
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V>,
    V: NeuronTrait,
    W: RandomizerTrait,
    X: DisplayControllerTrait,
>(
    genomes: &mut Vec<T>,
    randomizer: &mut W,
    display_controller: &X,
) -> Result<(), String> {
    display_controller
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

        display_controller
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
}
