extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::GenomeTrait;
use evolution::domain::genome_couple::GenomeCouple;
use evolution::domain::is_population_ordered_by_fitness::is_population_ordered_by_fitness;
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
    // Note: for the crossing over to make sense, it should be sorted in descending order of fitness. Otherwise
    // you are just crossing over the genomes it comes across first, and discards the rest.
    if !is_population_ordered_by_fitness(&population)? {
        panic!("Was going to create the next generation, but the population wasn't ordered by fitness.!");
    }

    let mut next_generation = Population::new();

    for index in 0..population.get_midpoint() {
        let (mut first_child, mut second_child) = crossover_genomes(
            GenomeCouple::new(
                &population.get_genomes()?[index as usize],
                &population.get_genomes()?[(index as usize) + 1],
            )?,
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

    // Give new identifiers to the genomes.
    (0..next_generation.get_size()).for_each(|index| {
        next_generation.get_genomes_mut().unwrap()[index as usize].set_identifier(index + 1);
    });

    assert!(!next_generation
        .get_genomes()?
        .iter()
        .any(|evolved_genome| evolved_genome.get_identifier() == 0));

    Ok(next_generation)
}
