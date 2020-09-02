extern crate serde;

use self::serde::{Deserialize, Serialize};
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;
use std::fmt;
use std::marker::PhantomData;

pub trait GenomeTrait<T: NeuralNetworkTrait<U>, U: NeuronTrait> {
    fn get_neural_network(&self) -> &T;
    fn get_neural_network_mut(&mut self) -> &mut T;
    fn get_fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Genome<T: NeuralNetworkTrait<U>, U: NeuronTrait> {
    neural_network: T,
    fitness: f64,
    phantom: PhantomData<U>,
}

impl<T: NeuralNetworkTrait<U>, U: NeuronTrait> Genome<T, U> {
    pub fn new(neural_network: T) -> Self
    where
        T: NeuralNetworkTrait<U>,
    {
        Genome {
            neural_network,
            fitness: 0f64,
            phantom: PhantomData,
        }
    }
}

impl<T: NeuralNetworkTrait<U> + std::fmt::Debug, U: NeuronTrait> fmt::Debug for Genome<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--Genome (fitness: {})--", self.fitness)?;
        writeln!(f, "--> Neural network:")?;
        writeln!(f, "{:#?}", self.neural_network)
    }
}

impl<T: NeuralNetworkTrait<U>, U: NeuronTrait> GenomeTrait<T, U> for Genome<T, U> {
    fn get_neural_network(&self) -> &T
    where
        T: NeuralNetworkTrait<U>,
    {
        &self.neural_network
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness
    }

    fn get_neural_network_mut(&mut self) -> &mut T {
        &mut self.neural_network
    }
}

#[cfg(test)]

mod tests {

    use neural_network::NeuralNetwork;
    extern crate randomization;
    use self::randomization::randomizer::Randomizer;
    use neuron::Neuron;
    use neuron_activation::activation_functions::ActivationFunctions;

    use super::*;

    #[test]
    fn test_after_creating_a_genome_it_has_expected_properties() -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let neural_network = NeuralNetwork::new_with_specified_layers(
            &[[4, 3], [3, 2], [2, 1]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

        let genome = Genome::new(neural_network);

        assert_eq!(genome.get_neural_network().get_number_of_layers(), 3);

        assert_eq!(genome.get_fitness(), 0f64);

        Ok(())
    }
}
