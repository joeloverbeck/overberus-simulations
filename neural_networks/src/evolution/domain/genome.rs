extern crate file_system;
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
    fn get_identifier(&self) -> u32;
    fn set_identifier(&mut self, identifier: u32);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Genome<T: NeuralNetworkTrait<U>, U: NeuronTrait> {
    identifier: u32,
    neural_network: T,
    fitness: f64,
    phantom: PhantomData<U>,
}

impl<T: NeuralNetworkTrait<U>, U: NeuronTrait> Genome<T, U> {
    pub fn new(identifier: u32, neural_network: T) -> Self
    where
        T: NeuralNetworkTrait<U>,
    {
        Genome {
            identifier,
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
    fn get_identifier(&self) -> u32 {
        self.identifier
    }

    fn set_identifier(&mut self, identifier: u32) {
        self.identifier = identifier;
    }
}

#[cfg(test)]

mod tests {

    use self::file_system::deserialize_json_from_string::deserialize_json_from_string;
    use self::file_system::read_file_to_string::read_file_to_string;
    use self::file_system::save_json::save_json;

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

        let genome = Genome::new(1, neural_network);

        assert_eq!(genome.get_neural_network().get_number_of_layers(), 3);

        assert_eq!(genome.get_fitness(), 0f64);

        Ok(())
    }

    #[test]
    fn test_can_serialize_and_deserialize_genomes() -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let neural_network = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[3, 2], [2, 2], [2, 2]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

        let genome = Genome::new(1, neural_network);

        use self::file_system::does_file_exist::does_file_exist;
        use self::file_system::remove_file::remove_file;

        let file_path = "./testdata/genome_test.json";

        assert!(
            !does_file_exist(file_path)?,
            "The file path {:?} shouldn't correspond to an existing file",
            file_path
        );

        save_json(file_path, &genome)?;

        let file_as_string = read_file_to_string(file_path)?;

        match deserialize_json_from_string::<Genome<NeuralNetwork<Neuron>, Neuron>>(&file_as_string)
        {
            Err(error) => {
                remove_file(file_path)?;
                panic!("Couldn't deserialize {:?}. Error: {:?}", file_path, error);
            }
            Ok(deserialized) => {
                remove_file(file_path)?;

                assert!(!does_file_exist(file_path)?, "After serializing to file and deserializing, the file path {:?} shouldn't correspond to an existing file", file_path);

                assert_eq!(deserialized.get_identifier(), 1);
            }
        }

        Ok(())
    }
}
