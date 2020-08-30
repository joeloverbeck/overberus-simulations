extern crate randomization;

use self::randomization::randomizer::{Randomizer, RandomizerTrait};
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;

pub trait PopulationTrait {
    type RandomizerType: RandomizerTrait;
    type NeuralNetworkType: NeuralNetworkTrait;

    fn new() -> Self;
    fn get_number_of_neural_networks(&self) -> u32;
    fn new_with_specified_layers(
        number_of_neural_networks: u32,
        layer_definition: &[[usize; 2]],
        randomizer: &mut Self::RandomizerType,
    ) -> Result<Self, String>
    where
        Self: std::marker::Sized;
    fn get_size(&self) -> u32;
    fn add(&mut self, neural_network: Self::NeuralNetworkType) -> Result<(), String>;
    fn get_neural_network(&self, index: usize) -> &Self::NeuralNetworkType;
}

pub struct Population {
    neural_networks: Vec<NeuralNetwork>,
    fitnesses: Vec<f64>,
}

impl PopulationTrait for Population {
    type RandomizerType = Randomizer;
    type NeuralNetworkType = NeuralNetwork;

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
        randomizer: &mut Self::RandomizerType,
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
        neural_network: Self::NeuralNetworkType,
    ) -> std::result::Result<(), std::string::String> {
        self.neural_networks.push(neural_network);

        self.fitnesses.push(0f64);

        Ok(())
    }
    fn get_neural_network(&self, index: usize) -> &Self::NeuralNetworkType {
        &self.neural_networks[index]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use layer::Layer;
    use layer::LayerTrait;

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

    #[test]
    fn test_the_first_neural_network_of_the_population_has_expected_properties(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let population =
            Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer)?;

        assert_eq!(population.get_neural_network(0).get_number_of_layers(), 3);
        assert_eq!(
            population
                .get_neural_network(0)
                .get_layer(0)
                .get_number_of_inputs(),
            4
        );
        assert_eq!(
            population
                .get_neural_network(0)
                .get_layer(0)
                .get_number_of_neurons(),
            3
        );

        Ok(())
    }

    #[test]
    fn test_the_last_neural_network_of_the_population_has_expected_properties() -> Result<(), String>
    {
        let mut randomizer = Randomizer::new();

        let population =
            Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer)?;

        assert_eq!(population.get_neural_network(9).get_number_of_layers(), 3);
        assert_eq!(
            population
                .get_neural_network(9)
                .get_layer(2)
                .get_number_of_inputs(),
            2
        );
        assert_eq!(
            population
                .get_neural_network(9)
                .get_layer(2)
                .get_number_of_neurons(),
            1
        );

        Ok(())
    }

    #[test]
    fn test_can_add_neural_networks_to_population_one_by_one() -> Result<(), String> {
        let mut population = Population::new();
        let mut neural_network1 = NeuralNetwork::new();

        let mut randomizer = Randomizer::new();

        let layer1 = Layer::new(3, 2, &mut randomizer);
        let layer2 = Layer::new(2, 1, &mut randomizer);

        neural_network1.add(layer1)?;
        neural_network1.add(layer2)?;

        population.add(neural_network1)?;

        assert_eq!(population.get_size(), 1);

        let mut neural_network2 = NeuralNetwork::new();

        let layer1 = Layer::new(3, 2, &mut randomizer);
        let layer2 = Layer::new(2, 1, &mut randomizer);

        neural_network2.add(layer1)?;
        neural_network2.add(layer2)?;

        population.add(neural_network2)?;

        assert_eq!(population.get_size(), 2);

        assert_eq!(population.get_number_of_neural_networks(), 2);

        Ok(())
    }
}
