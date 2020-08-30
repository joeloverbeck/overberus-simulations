extern crate randomization;

use self::randomization::randomizer::{Randomizer, RandomizerTrait};
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;

pub trait PopulationTrait {
    type Randomizer: RandomizerTrait;
    type NeuralNetwork: NeuralNetworkTrait;

    fn new() -> Self;
    fn get_number_of_neural_networks(&self) -> u32;
    fn new_with_specified_layers(
        number_of_neural_networks: u32,
        layer_definition: &[[usize; 2]],
        randomizer: &mut Self::Randomizer,
    ) -> Result<Self, String>
    where
        Self: std::marker::Sized;
    fn get_size(&self) -> u32;
    fn add(&mut self, neural_network: Self::NeuralNetwork) -> Result<(), String>;
}

pub struct Population {
    neural_networks: Vec<NeuralNetwork>,
    fitnesses: Vec<f64>,
}

impl PopulationTrait for Population {
    type Randomizer = Randomizer;
    type NeuralNetwork = NeuralNetwork;

    fn new() -> Self {
        Population {
            neural_networks: Vec::new(),
            fitnesses: Vec::new(),
        }
    }

    fn get_number_of_neural_networks(&self) -> u32 {
        self.neural_networks.len() as u32
    }

    fn new_with_specified_layers(
        number_of_neural_networks: u32,
        layer_definition: &[[usize; 2]],
        randomizer: &mut Self::Randomizer,
    ) -> Result<Self, String>
    where
        Self: std::marker::Sized,
    {
        let mut population = Population::new();

        for _ in 0..number_of_neural_networks {
            population.add(NeuralNetwork::new_with_specified_layers(
                layer_definition,
                randomizer,
            ))?;
        }

        Ok(population)
    }

    fn get_size(&self) -> u32 {
        self.neural_networks.len() as u32
    }

    fn add(
        &mut self,
        neural_network: Self::NeuralNetwork,
    ) -> std::result::Result<(), std::string::String> {
        self.neural_networks.push(neural_network);

        self.fitnesses.push(0f64);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_can_create_empty_population_of_neural_networks() -> Result<(), String> {
        let population = Population::new();

        assert_eq!(population.get_number_of_neural_networks(), 0);

        Ok(())
    }

    #[test]
    fn test_can_create_population_of_x_number_of_neural_networks_with_prespecified_layers(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let population_result =
            Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer);

        if let Err(error) = population_result {
            panic!(
                "Failed to create a population with specified layers: {:?}",
                error
            );
        }

        let population = population_result.unwrap();

        assert_eq!(population.get_size(), 10);

        Ok(())
    }
}
