extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use layer::LayerTrait;

use layer::Layer;
use neuron::NeuronTrait;

pub fn produce_child_for_crossover<T: NeuronTrait, U: RandomizerTrait>(
    parent: &Layer<T>,
    randomizer: &mut U,
    neuron_creator: fn(u32, &mut U) -> T,
) -> Result<Layer<T>, String> {
    Ok(Layer::<T>::create_layer(
        parent.get_number_of_inputs(),
        parent.get_number_of_neurons() as u32,
        randomizer,
        neuron_creator,
    ))
}
