use neuron_activation::sigmoid::sigmoid;

pub fn swish(z: f64) -> f64 {
    z * sigmoid(z)
}
