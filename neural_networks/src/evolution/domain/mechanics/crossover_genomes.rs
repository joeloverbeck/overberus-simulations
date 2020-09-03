extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::GenomeTrait;
use evolution::domain::genome_couple::GenomeCouple;
use evolution::domain::layer_couple::LayerCouple;
use evolution::domain::mechanics::crossover_layers::crossover_layers;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

type ResultCrossoverGenomes<T> = Result<(T, T), String>;

pub fn crossover_genomes<
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
    W: RandomizerTrait,
    X: Fn(u32, U) -> T,
    Y: Fn() -> U,
    Z: Fn(u32, &mut W) -> V,
>(
    couple: GenomeCouple<T, U, V>,
    genome_creator: &X,
    neural_network_creator: &Y,
    neuron_creator: Z,
    randomizer: &mut W,
) -> ResultCrossoverGenomes<T> {
    let mut first_child = neural_network_creator();
    let mut second_child = neural_network_creator();

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
            &neuron_creator,
        )?;
        first_child.add(c1)?;
        second_child.add(c2)?;
    }

    Ok((
        genome_creator(couple.get_first_parent().get_identifier(), first_child),
        genome_creator(couple.get_second_parent().get_identifier(), second_child),
    ))
}
