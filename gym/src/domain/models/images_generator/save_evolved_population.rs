extern crate file_system;
extern crate neural_networks;
extern crate randomization;
extern crate serde;

use self::file_system::save_json::save_json;
use self::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::evolution::domain::population::PopulationTrait;
use self::neural_networks::neural_network::NeuralNetworkTrait;
use self::neural_networks::neuron::NeuronTrait;
use self::serde::Serialize;

pub fn save_evolved_population<
    T: GenomeTrait<U, V> + Clone + Serialize,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
>(
    evolved_population: &Population<T, U, V>,
) -> Result<(), String> {
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
}
