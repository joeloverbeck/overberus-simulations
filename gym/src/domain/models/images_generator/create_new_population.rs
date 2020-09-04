extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::randomization::randomizer::RandomizerTrait;

pub fn create_new_population<
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
    Y: RandomizerTrait,
    Z: Fn(&mut Y) -> Result<Population<T, U, V>, String>,
>(
    defined_population_creator: Z,
    randomizer: &mut Y,
) -> Result<Option<Population<T, U, V>>, String> {
    if let Ok(population) = defined_population_creator(randomizer) {
        return Ok(Some(population));
    }

    Err("Failed to create a new population for the images generation program.".to_string())
}
