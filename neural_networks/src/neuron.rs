extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use activation_functions::sigmoid::sigmoid;

pub trait NeuronTrait {
    fn new<T: RandomizerTrait>(number_of_inputs: u32, randomizer: &mut T) -> Self;
    fn get_number_of_weights(&self) -> u32;
    fn activate(&self, inputs: &[f64]) -> Result<f64, String>;
}

pub struct Neuron {
    weights: Vec<f64>,
    bias: f64,
}

impl NeuronTrait for Neuron {
    fn new<T>(number_of_inputs: u32, randomizer: &mut T) -> Self
    where
        T: RandomizerTrait,
    {
        Neuron {
            weights: (0..number_of_inputs)
                .map(|_| randomizer.get_normal())
                .collect(),
            bias: randomizer.get_normal(),
        }
    }

    fn get_number_of_weights(&self) -> u32 {
        self.weights.len() as u32
    }

    fn activate(&self, inputs: &[f64]) -> std::result::Result<f64, std::string::String> {
        Ok(sigmoid(
            self.weights
                .iter()
                .zip(inputs.iter())
                .map(|(w, x)| w * x)
                .sum::<f64>()
                + self.bias,
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_after_creating_a_neuron_it_has_expected_properties() -> Result<(), String> {
        let number_of_inputs = 3;

        struct FakeRandomizer {}

        impl RandomizerTrait for FakeRandomizer {
            fn get_normal(&mut self) -> f64 {
                0.4_f64
            }
        }

        let mut randomizer = FakeRandomizer {};

        let neuron = Neuron::new(number_of_inputs, &mut randomizer);

        let inputs = vec![0.0_f64, 1.0_f64, 0.0_f64];

        assert_eq!(neuron.get_number_of_weights(), 3);

        let activation = neuron.activate(&inputs)?;

        assert!(
            activation >= 0.6899,
            "Activation should be higher than 0.6899, but was {:?}",
            activation
        );
        assert!(activation <= 0.6900);

        Ok(())
    }
}
