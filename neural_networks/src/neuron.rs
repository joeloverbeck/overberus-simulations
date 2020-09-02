extern crate randomization;
extern crate serde;

use self::randomization::randomizer::RandomizerTrait;
use evolution::domain::constants::MUTATION_PROBABILITY;
use neuron_activation::activate_neuron::activate_neuron;
use neuron_activation::activation_functions::ActivationFunctions;

use self::serde::{Deserialize, Serialize};
use std::fmt;

pub trait NeuronTrait {
    fn new<T: RandomizerTrait>(
        number_of_inputs: u32,
        activation_function: ActivationFunctions,
        randomizer: &mut T,
    ) -> Self;
    fn get_number_of_weights(&self) -> u32;
    fn activate(&self, inputs: &[f64]) -> Result<f64, String>;
    fn get_bias(&self) -> f64;
    fn set_bias(&mut self, bias: f64);
    fn get_weight(&self, index: usize) -> Result<f64, String>;
    fn set_weight(&mut self, index: usize, weight: f64) -> Result<(), String>;
    fn get_activation_function(&self) -> &ActivationFunctions;
    fn set_activation_function(
        &mut self,
        activation_function: ActivationFunctions,
    ) -> Result<(), String>;
    fn should_mutate<T: RandomizerTrait>(randomizer: &mut T) -> Result<bool, String>;
    fn mutate<T: RandomizerTrait>(&mut self, randomizer: &mut T) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Neuron {
    weights: Vec<f64>,
    bias: f64,
    activation_function: ActivationFunctions,
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
    fn new<T>(
        number_of_inputs: u32,
        activation_function: ActivationFunctions,
        randomizer: &mut T,
    ) -> Self
    where
        T: RandomizerTrait,
    {
        Neuron {
            weights: (0..number_of_inputs)
                .map(|_| randomizer.get_normal())
                .collect(),
            bias: randomizer.get_normal(),
            activation_function,
        }
    }

    fn get_number_of_weights(&self) -> u32 {
        self.weights.len() as u32
    }

    fn activate(&self, inputs: &[f64]) -> std::result::Result<f64, std::string::String> {
        Ok(activate_neuron(
            self.weights
                .iter()
                .zip(inputs.iter())
                .map(|(w, x)| w * x)
                .sum::<f64>()
                + self.bias,
            &self.activation_function,
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
        self.weights[index] = weight;

        Ok(())
    }

    fn should_mutate<T>(randomizer: &mut T) -> std::result::Result<bool, std::string::String>
    where
        T: RandomizerTrait,
    {
        Ok(randomizer.generate_float_from_0_to_1() > 1f64 - MUTATION_PROBABILITY)
    }

    fn mutate<T>(&mut self, randomizer: &mut T) -> std::result::Result<(), std::string::String>
    where
        T: RandomizerTrait,
    {
        if Neuron::should_mutate(randomizer)? {
            self.bias = randomizer.get_normal();
        }

        // Ask for the possibility of mutation for each weight in this neuron.
        for index in 0..self.weights.len() {
            if Neuron::should_mutate(randomizer)? {
                // Mutate corresponding weight.
                self.weights[index] = randomizer.get_normal();
            }
        }

        Ok(())
    }
    fn get_activation_function(&self) -> &ActivationFunctions {
        &self.activation_function
    }
    fn set_activation_function(
        &mut self,
        activation_function: ActivationFunctions,
    ) -> std::result::Result<(), std::string::String> {
        self.activation_function = activation_function;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use neuron_activation::activation_functions::ActivationFunctions;

    #[test]
    fn test_after_creating_a_neuron_it_has_expected_properties() -> Result<(), String> {
        let number_of_inputs = 3;

        struct FakeRandomizer {}

        impl RandomizerTrait for FakeRandomizer {
            fn get_normal(&mut self) -> f64 {
                0.4_f64
            }
            fn generate_float_from_0_to_1(&mut self) -> f64 {
                todo!()
            }
        }

        let mut randomizer = FakeRandomizer {};

        let neuron = Neuron::new(
            number_of_inputs,
            ActivationFunctions::Sigmoid,
            &mut randomizer,
        );

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
