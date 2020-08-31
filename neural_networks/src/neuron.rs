extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use activation_functions::sigmoid::sigmoid;

use std::fmt;

pub trait NeuronTrait {
    fn new<T: RandomizerTrait>(number_of_inputs: u32, randomizer: &mut T) -> Self;
    fn get_number_of_weights(&self) -> u32;
    fn activate(&self, inputs: &[f64]) -> Result<f64, String>;
    fn get_bias(&self) -> f64;
    fn set_bias(&mut self, bias: f64);
    fn get_weight(&self, index: usize) -> Result<f64, String>;
    fn set_weight(&mut self, index: usize, weight: f64) -> Result<(), String>;
}

pub struct Neuron {
    weights: Vec<f64>,
    bias: f64,
}

impl fmt::Debug for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--Neuron (bias: {})--", self.bias)?;

        writeln!(f, "--> Weights:")?;

        for weight in &self.weights {
            writeln!(f, "    {:#?}", weight)?;
        }

        write!(f, "")
    }
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
    fn set_bias(&mut self, bias: f64) {
        self.bias = bias
    }
    fn get_bias(&self) -> f64 {
        self.bias
    }
    fn get_weight(&self, index: usize) -> std::result::Result<f64, std::string::String> {
        Ok(self.weights[index])
    }
    fn set_weight(
        &mut self,
        index: usize,
        weight: f64,
    ) -> std::result::Result<(), std::string::String> {
        Ok(self.weights[index] = weight)
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
            fn generate_f64(&mut self) -> f64 {
                todo!()
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
