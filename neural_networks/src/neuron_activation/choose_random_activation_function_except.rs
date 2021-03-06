extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use neuron_activation::activation_functions::ActivationFunctions;
use neuron_activation::choose_random_activation_function::choose_random_activation_function;

pub fn choose_random_activation_function_except<T: RandomizerTrait>(
    randomizer: &mut T,
    exceptions: &[ActivationFunctions],
) -> ActivationFunctions {
    loop {
        let choice = choose_random_activation_function(randomizer);

        if !exceptions.iter().any(|exception| exception == &choice) {
            return choice;
        }
    }
}
