extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use layer::Layer;
use layer::LayerTrait;

use neuron::NeuronTrait;

pub fn crossover_biases_of_neurons<T: NeuronTrait, U: RandomizerTrait>(
    first_parent: &Layer<T>,
    second_parent: &Layer<T>,
    first_child: &mut Layer<T>,
    second_child: &mut Layer<T>,
    index: usize,
    randomizer: &mut U,
) -> Result<(), String> {
    if Layer::<T>::should_crossover(randomizer)? {
        first_child
            .get_neuron_mut(index)?
            .set_bias(second_parent.get_neuron(index)?.get_bias());
        second_child
            .get_neuron_mut(index)?
            .set_bias(first_parent.get_neuron(index)?.get_bias());
    } else {
        first_child
            .get_neuron_mut(index)?
            .set_bias(first_parent.get_neuron(index)?.get_bias());
        second_child
            .get_neuron_mut(index)?
            .set_bias(second_parent.get_neuron(index)?.get_bias());
    }

    Ok(())
}
