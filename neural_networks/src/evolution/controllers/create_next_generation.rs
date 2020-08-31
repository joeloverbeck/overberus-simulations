extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::controllers::crossover_genomes::crossover_genomes;
use evolution::controllers::mutate_genome::mutate_genome;
use evolution::domain::genome::Genome;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetwork;

pub fn create_next_generation<T: RandomizerTrait>(
    population: &Population<Genome<NeuralNetwork>, NeuralNetwork>,
    randomizer: &mut T,
) -> Result<Population<Genome<NeuralNetwork>, NeuralNetwork>, String> {
    let mut next_generation = Population::new();

    let sorted_index = population.get_sorted_index();

    let mid = population.get_size() % 2 + population.get_size() / 2;

    for index in 0..mid {
        let first_parent = population.get_genome(sorted_index[index as usize])?;
        let second_parent = population.get_genome(sorted_index[(index + 1) as usize])?;

        let (mut first_child, mut second_child) =
            crossover_genomes(first_parent, second_parent, randomizer)?;

        mutate_genome(&mut first_child, randomizer)?;
        mutate_genome(&mut second_child, randomizer)?;

        next_generation.add(first_child)?;

        if !(index == mid - 1 && population.get_size() % 2 == 1) {
            next_generation.add(second_child)?;
        }
    }

    Ok(next_generation)
}
