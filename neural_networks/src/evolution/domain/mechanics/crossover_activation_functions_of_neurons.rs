extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::layer_couple::LayerCouple;
use layer::Layer;
use layer::LayerTrait;

use neuron::NeuronTrait;

pub fn crossover_activation_functions_of_neurons<T: NeuronTrait, U: RandomizerTrait>(
    layer_couple: &LayerCouple<T>,
    first_child: &mut Layer<T>,
    second_child: &mut Layer<T>,
    index: usize,
    randomizer: &mut U,
) -> Result<(), String> {
    if Layer::<T>::should_crossover(randomizer)? {
        first_child.get_neuron_mut(index)?.set_activation_function(
            *layer_couple
                .get_second_parent()
                .get_neuron(index)?
                .get_activation_function(),
        )?;
        second_child
            .get_neuron_mut(index)?
            .set_activation_function(
                *layer_couple
                    .get_first_parent()
                    .get_neuron(index)?
                    .get_activation_function(),
            )?;
    } else {
        first_child.get_neuron_mut(index)?.set_activation_function(
            *layer_couple
                .get_first_parent()
                .get_neuron(index)?
                .get_activation_function(),
        )?;
        second_child
            .get_neuron_mut(index)?
            .set_activation_function(
                *layer_couple
                    .get_second_parent()
                    .get_neuron(index)?
                    .get_activation_function(),
            )?;
    }

    Ok(())
}
