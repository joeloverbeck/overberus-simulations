extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::controllers::crossover_weights_of_neuron::crossover_weights_of_neuron;
use layer::Layer;
use layer::LayerTrait;
use neuron::Neuron;
use neuron::NeuronTrait;

pub fn crossover_layers<T: RandomizerTrait>(
    first_parent: &Layer<Neuron>,
    second_parent: &Layer<Neuron>,
    randomizer: &mut T,
) -> Result<(Layer<Neuron>, Layer<Neuron>), String> {
    let mut first_child = Layer::new(
        first_parent.get_number_of_inputs(),
        first_parent.get_number_of_neurons() as u32,
        randomizer,
    );
    let mut second_child = Layer::new(
        first_parent.get_number_of_inputs(),
        first_parent.get_number_of_neurons() as u32,
        randomizer,
    );

    // Cannot iter() over here since destructuring assignments are not allowed
    // https://github.com/rust-lang/rfcs/issues/372

    for index in 0..first_parent.get_number_of_neurons() as usize {
        if Layer::<Neuron>::should_crossover(randomizer)? {
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

        crossover_weights_of_neuron(
            first_parent,
            second_parent,
            &mut first_child,
            &mut second_child,
            index,
            randomizer,
        )?;
    }

    Ok((first_child, second_child))
}
