extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::layer_couple::LayerCouple;
use layer::Layer;
use layer::LayerTrait;
use neuron::NeuronTrait;

pub fn crossover_weights_of_neurons<T: NeuronTrait, U: RandomizerTrait>(
    layer_couple: &LayerCouple<T>,
    first_child: &mut Layer<T>,
    second_child: &mut Layer<T>,
    index: usize,
    randomizer: &mut U,
) -> Result<(), String> {
    for j in 0..layer_couple.get_first_parent().get_number_of_inputs() as usize {
        if Layer::<T>::should_crossover(randomizer)? {
            first_child.get_neuron_mut(index)?.set_weight(
                j,
                layer_couple
                    .get_second_parent()
                    .get_neuron(index)?
                    .get_weight(j)?,
            )?;
            second_child.get_neuron_mut(index)?.set_weight(
                j,
                layer_couple
                    .get_first_parent()
                    .get_neuron(index)?
                    .get_weight(j)?,
            )?;
        } else {
            first_child.get_neuron_mut(index)?.set_weight(
                j,
                layer_couple
                    .get_first_parent()
                    .get_neuron(index)?
                    .get_weight(j)?,
            )?;
            second_child.get_neuron_mut(index)?.set_weight(
                j,
                layer_couple
                    .get_second_parent()
                    .get_neuron(index)?
                    .get_weight(j)?,
            )?;
        }
    }

    Ok(())
}
