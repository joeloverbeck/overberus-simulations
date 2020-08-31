extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::Genome;
use evolution::domain::genome::GenomeTrait;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;

pub fn crossover_genomes<T: RandomizerTrait>(
    first_genome: &Genome<NeuralNetwork>,
    second_genome: &Genome<NeuralNetwork>,
    randomizer: &mut T,
) -> Result<(Genome<NeuralNetwork>, Genome<NeuralNetwork>), String> {
    let mut first_child = NeuralNetwork::new();
    let mut second_child = NeuralNetwork::new();

    for (first_parent, second_parent) in first_genome
        .get_neural_network()
        .get_layers()
        .iter()
        .zip(second_genome.get_neural_network().get_layers().iter())
    {
        let (c1, c2) = first_parent.crossover(second_parent, randomizer)?;
        first_child.add(c1)?;
        second_child.add(c2)?;
    }

    Ok((Genome::new(first_child), Genome::new(second_child)))
}
