use neuron_activation::activation_functions::ActivationFunctions;
use neuron_activation::sigmoid::sigmoid;

pub fn activate_neuron(value: f64, activation_function: &ActivationFunctions) -> f64 {
    match activation_function {
        ActivationFunctions::Sigmoid => sigmoid(value),
    }
}
