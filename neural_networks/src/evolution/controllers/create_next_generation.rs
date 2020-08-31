extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::controllers::crossover_genomes::crossover_genomes;
use evolution::controllers::mutate_genome::mutate_genome;
use evolution::domain::genome::Genome;
use evolution::domain::genome_couple::GenomeCouple;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetwork;
use neuron::Neuron;

type NN = NeuralNetwork<Neuron>;
type GN = Genome<NN, Neuron>;

pub fn create_next_generation<T: RandomizerTrait>(
    population: &Population<GN, NN, Neuron>,
    randomizer: &mut T,
) -> Result<Population<GN, NN, Neuron>, String> {
    let mut next_generation = Population::new();

    for index in 0..population.get_midpoint() {
        let (mut first_child, mut second_child) =
            crossover_genomes(GenomeCouple::new(index, population)?, randomizer)?;

        mutate_genome(&mut first_child, randomizer)?;
        mutate_genome(&mut second_child, randomizer)?;

        next_generation.add(first_child)?;

        if !(index == population.get_midpoint() - 1 && population.get_size() % 2 == 1) {
            next_generation.add(second_child)?;
        }
    }

    Ok(next_generation)
}
