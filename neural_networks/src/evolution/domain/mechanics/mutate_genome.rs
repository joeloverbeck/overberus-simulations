extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::genome::GenomeTrait;
use layer::LayerTrait;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

pub fn mutate_genome<
    T: GenomeTrait<NeuralNetwork<U>, U>,
    U: NeuronTrait + Clone,
    V: RandomizerTrait,
>(
    genome: &mut T,
    randomizer: &mut V,
) -> Result<(), String> {
    // This only delegates to the neural network.
    for layer in genome.get_neural_network_mut().get_layers_mut().iter_mut() {
        for neuron in layer.get_neurons_mut() {
            neuron.mutate(randomizer)?;
        }
    }

    Ok(())
}
