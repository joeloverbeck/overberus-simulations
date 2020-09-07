use evolution::domain::genome::GenomeTrait;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

pub fn is_population_ordered_by_fitness<
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
>(
    population: &Population<T, U, V>,
) -> Result<bool, String> {
    let highest_fitness = population.get_genomes()?[0].get_fitness();

    for genome in population.get_genomes()?.iter() {
        if genome.get_fitness() > highest_fitness {
            return Ok(false);
        }
    }

    Ok(true)
}
