use neuron_activation::activation_functions::ActivationFunctions;
use neuron_activation::cosine::cosine;
use neuron_activation::relu::relu;
use neuron_activation::sigmoid::sigmoid;
use neuron_activation::sinusoid::sinusoid;
use neuron_activation::softplus::softplus;
use neuron_activation::swish::swish;
use neuron_activation::tanh::tanh;

pub fn activate_neuron(value: f64, activation_function: &ActivationFunctions) -> f64 {
    match activation_function {
        ActivationFunctions::Sigmoid => sigmoid(value),
        ActivationFunctions::Softplus => softplus(value),
        ActivationFunctions::Relu => relu(value),
        ActivationFunctions::Sinusoid => sinusoid(value),
        ActivationFunctions::Tanh => tanh(value),
        ActivationFunctions::Cosine => cosine(value),
        ActivationFunctions::Swish => swish(value),
    }
}
