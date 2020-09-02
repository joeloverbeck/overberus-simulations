extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use neuron_activation::activation_functions::ActivationFunctions;

pub fn choose_random_activation_function<T: RandomizerTrait>(
    randomizer: &mut T,
) -> ActivationFunctions {
    randomizer.choose_random_from_vec(&[
        ActivationFunctions::Sigmoid,
        ActivationFunctions::Relu,
        ActivationFunctions::Softplus,
        ActivationFunctions::Sinusoid,
        ActivationFunctions::Cosine,
        ActivationFunctions::Tanh,
        ActivationFunctions::Swish,
    ])
}
