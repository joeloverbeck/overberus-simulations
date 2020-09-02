extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::GenomeTrait;
use layer::LayerTrait;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

pub fn mutate_genome<
    T: GenomeTrait<U, V>,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
    W: RandomizerTrait,
>(
    genome: &mut T,
    randomizer: &mut W,
) -> Result<(), String> {
    // This only delegates to the neural network.
    for layer in genome.get_neural_network_mut().get_layers_mut().iter_mut() {
        for neuron in layer.get_neurons_mut() {
            neuron.mutate(randomizer)?;
        }
    }

    Ok(())
}
