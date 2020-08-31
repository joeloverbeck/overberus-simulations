extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use layer::Layer;
use layer::LayerTrait;
use neuron::Neuron;
use neuron::NeuronTrait;

pub fn crossover_weights_of_neuron<T: RandomizerTrait>(
    first_parent: &Layer<Neuron>,
    second_parent: &Layer<Neuron>,
    first_child: &mut Layer<Neuron>,
    second_child: &mut Layer<Neuron>,
    index: usize,
    randomizer: &mut T,
) -> Result<(), String> {
    for j in 0..first_parent.get_number_of_inputs() as usize {
        if Layer::<Neuron>::should_crossover(randomizer)? {
            first_child
                .get_neuron_mut(index)?
                .set_weight(j, second_parent.get_neuron(index)?.get_weight(j)?)?;
            second_child
                .get_neuron_mut(index)?
                .set_weight(j, first_parent.get_neuron(index)?.get_weight(j)?)?;
        } else {
            first_child
                .get_neuron_mut(index)?
                .set_weight(j, first_parent.get_neuron(index)?.get_weight(j)?)?;
            second_child
                .get_neuron_mut(index)?
                .set_weight(j, second_parent.get_neuron(index)?.get_weight(j)?)?;
        }
    }

    Ok(())
}
