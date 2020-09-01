extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::layer_couple::LayerCouple;
use evolution::domain::mechanics::crossover_biases_of_neurons::crossover_biases_of_neurons;
use evolution::domain::mechanics::crossover_weights_of_neurons::crossover_weights_of_neurons;
use evolution::domain::mechanics::produce_child_for_crossover::produce_child_for_crossover;
use layer::Layer;
use layer::LayerTrait;
use neuron::NeuronTrait;

pub fn crossover_layers<T: NeuronTrait, U: RandomizerTrait>(
    layer_couple: LayerCouple<T>,
    randomizer: &mut U,
    neuron_creator: fn(u32, &mut U) -> T,
) -> Result<(Layer<T>, Layer<T>), String> {
    let mut first_child =
        produce_child_for_crossover(layer_couple.get_first_parent(), randomizer, neuron_creator)?;
    let mut second_child =
        produce_child_for_crossover(layer_couple.get_first_parent(), randomizer, neuron_creator)?;

    // Cannot iter() over here since destructuring assignments are not allowed
    // https://github.com/rust-lang/rfcs/issues/372

    for index in 0..layer_couple.get_first_parent().get_number_of_neurons() as usize {
        crossover_biases_of_neurons(
            &layer_couple,
            &mut first_child,
            &mut second_child,
            index,
            randomizer,
        )?;

        crossover_weights_of_neurons(
            &layer_couple,
            &mut first_child,
            &mut second_child,
            index,
            randomizer,
        )?;
    }

    Ok((first_child, second_child))
}
