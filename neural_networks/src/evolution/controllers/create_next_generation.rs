extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::GenomeTrait;
use evolution::domain::genome_couple::GenomeCouple;
use evolution::domain::mechanics::crossover_genomes::crossover_genomes;
use evolution::domain::mechanics::mutate_genome::mutate_genome;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

pub fn create_next_generation<
    T: GenomeTrait<V, W> + Clone,
    V: NeuralNetworkTrait<W> + Clone,
    W: NeuronTrait + Clone,
    X: RandomizerTrait,
    Y: Fn(u32, V) -> T,
    Z: Fn() -> V,
    A: Fn(u32, &mut X) -> W,
>(
    population: &Population<T, V, W>,
    genome_creator: Y,
    neural_network_creator: Z,
    neuron_creator: A,
    randomizer: &mut X,
) -> Result<Population<T, V, W>, String> {
    let mut next_generation = Population::new();

    for index in 0..population.get_midpoint() {
        let (mut first_child, mut second_child) = crossover_genomes(
            GenomeCouple::new(index, population)?,
            &genome_creator,
            &neural_network_creator,
            &neuron_creator,
            randomizer,
        )?;

        mutate_genome(&mut first_child, randomizer)?;
        mutate_genome(&mut second_child, randomizer)?;

        next_generation.add(first_child)?;

        if !(index == population.get_midpoint() - 1 && population.get_size() % 2 == 1) {
            next_generation.add(second_child)?;
        }
    }

    Ok(next_generation)
}
