extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::Genome;
use evolution::domain::genome_couple::GenomeCouple;
use evolution::domain::mechanics::crossover_genomes::crossover_genomes;
use evolution::domain::mechanics::mutate_genome::mutate_genome;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

type GN<T, U> = Genome<T, U>;

pub fn create_next_generation<
    T: NeuralNetworkTrait<U> + Clone,
    U: NeuronTrait + Clone,
    V: RandomizerTrait,
    W: Fn() -> T,
>(
    population: &Population<GN<T, U>, T, U>,
    neural_network_creator: W,
    neuron_creator: fn(u32, &mut V) -> U,
    randomizer: &mut V,
) -> Result<Population<GN<T, U>, T, U>, String> {
    let mut next_generation = Population::new();

    for index in 0..population.get_midpoint() {
        let (mut first_child, mut second_child) = crossover_genomes(
            GenomeCouple::new(index, population)?,
            &neural_network_creator,
            neuron_creator,
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
