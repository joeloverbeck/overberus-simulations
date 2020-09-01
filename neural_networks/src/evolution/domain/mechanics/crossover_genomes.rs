extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::Genome;
use evolution::domain::genome::GenomeTrait;
use evolution::domain::genome_couple::GenomeCouple;
use evolution::domain::layer_couple::LayerCouple;
use evolution::domain::mechanics::crossover_layers::crossover_layers;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

type GN<T> = Genome<NeuralNetwork<T>, T>;

pub fn crossover_genomes<T: NeuronTrait + Clone, U: RandomizerTrait>(
    couple: GenomeCouple<T>,
    randomizer: &mut U,
    neuron_creator: fn(u32, &mut U) -> T,
) -> Result<(GN<T>, GN<T>), String> {
    let mut first_child = NeuralNetwork::new();
    let mut second_child = NeuralNetwork::new();

    for (first_layer, second_layer) in couple
        .get_first_parent()
        .get_neural_network()
        .get_layers()
        .iter()
        .zip(
            couple
                .get_second_parent()
                .get_neural_network()
                .get_layers()
                .iter(),
        )
    {
        let (c1, c2) = crossover_layers(
            LayerCouple::new(first_layer, second_layer)?,
            randomizer,
            neuron_creator,
        )?;
        first_child.add(c1)?;
        second_child.add(c2)?;
    }

    Ok((Genome::new(first_child), Genome::new(second_child)))
}
