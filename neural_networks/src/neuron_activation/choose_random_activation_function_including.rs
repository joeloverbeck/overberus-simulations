extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use neuron_activation::activation_functions::ActivationFunctions;
use neuron_activation::choose_random_activation_function::choose_random_activation_function;

pub fn choose_random_activation_function_including<T: RandomizerTrait>(
    randomizer: &mut T,
    including: &[ActivationFunctions],
) -> ActivationFunctions {
    loop {
        let choice = choose_random_activation_function(randomizer);

        if including.iter().any(|exception| exception == &choice) {
            return choice;
        }
    }
}
