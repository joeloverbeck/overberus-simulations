extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use layer::LayerTrait;
use neuron::Neuron;

use layer::Layer;
use neuron::NeuronTrait;

pub fn produce_child_for_crossover<T: NeuronTrait, U: RandomizerTrait>(
    parent: &Layer<T>,
    randomizer: &mut U,
) -> Result<Layer<Neuron>, String> {
    Ok(Layer::new(
        parent.get_number_of_inputs(),
        parent.get_number_of_neurons() as u32,
        randomizer,
    ))
}
