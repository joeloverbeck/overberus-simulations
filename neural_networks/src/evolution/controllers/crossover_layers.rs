extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::controllers::crossover_biases_of_neurons::crossover_biases_of_neurons;
use evolution::controllers::crossover_weights_of_neurons::crossover_weights_of_neurons;
use evolution::controllers::produce_child_for_crossover::produce_child_for_crossover;
use layer::Layer;
use layer::LayerTrait;
use neuron::Neuron;

pub fn crossover_layers<T: RandomizerTrait>(
    first_parent: &Layer<Neuron>,
    second_parent: &Layer<Neuron>,
    randomizer: &mut T,
) -> Result<(Layer<Neuron>, Layer<Neuron>), String> {
    let mut first_child = produce_child_for_crossover(first_parent, randomizer)?;
    let mut second_child = produce_child_for_crossover(first_parent, randomizer)?;

    // Cannot iter() over here since destructuring assignments are not allowed
    // https://github.com/rust-lang/rfcs/issues/372

    for index in 0..first_parent.get_number_of_neurons() as usize {
        crossover_biases_of_neurons(
            first_parent,
            second_parent,
            &mut first_child,
            &mut second_child,
            index,
            randomizer,
        )?;

        crossover_weights_of_neurons(
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
