extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use layer::Layer;
use layer::LayerTrait;
use neuron::NeuronTrait;

pub fn crossover_layers<T: RandomizerTrait>(
    first: &Layer,
    second: &Layer,
    randomizer: &mut T,
) -> Result<(Layer, Layer), String> {
    let mut first_child = Layer::new(
        first.get_number_of_inputs(),
        first.get_number_of_neurons() as u32,
        randomizer,
    );
    let mut second_child = Layer::new(
        first.get_number_of_inputs(),
        first.get_number_of_neurons() as u32,
        randomizer,
    );

    // Cannot iter() over here since destructuring assignments are not allowed
    // https://github.com/rust-lang/rfcs/issues/372

    for index in 0..first.get_number_of_neurons() as usize {
        if Layer::should_crossover(randomizer)? {
            first_child
                .get_neuron_mut(index)?
                .set_bias(second.get_neuron(index)?.get_bias());
            second_child
                .get_neuron_mut(index)?
                .set_bias(first.get_neuron(index)?.get_bias());
        } else {
            first_child
                .get_neuron_mut(index)?
                .set_bias(first.get_neuron(index)?.get_bias());
            second_child
                .get_neuron_mut(index)?
                .set_bias(second.get_neuron(index)?.get_bias());
        }

        for j in 0..first.get_number_of_inputs() as usize {
            if Layer::should_crossover(randomizer)? {
                first_child
                    .get_neuron_mut(index)?
                    .set_weight(j, second.get_neuron(index)?.get_weight(j)?)?;
                second_child
                    .get_neuron_mut(index)?
                    .set_weight(j, first.get_neuron(index)?.get_weight(j)?)?;
            } else {
                first_child
                    .get_neuron_mut(index)?
                    .set_weight(j, first.get_neuron(index)?.get_weight(j)?)?;
                second_child
                    .get_neuron_mut(index)?
                    .set_weight(j, second.get_neuron(index)?.get_weight(j)?)?;
            }
        }
    }

    Ok((first_child, second_child))
}
