extern crate randomization;
use self::randomization::randomizer::RandomizerTrait;

use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use std::fmt;

pub trait GenomeTrait<T: NeuralNetworkTrait> {
    fn new(neural_network: T) -> Self;
    fn get_neural_network(&self) -> &T;
    fn get_fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
    fn mutate(&mut self) -> Result<(), String>;
}

pub struct Genome<T: NeuralNetworkTrait> {
    neural_network: T,
    fitness: f64,
}

impl Genome<NeuralNetwork> {
    pub fn crossover<T: RandomizerTrait>(
        &self,
        other: &Genome<NeuralNetwork>,
        randomizer: &mut T,
    ) -> Result<(Genome<NeuralNetwork>, Genome<NeuralNetwork>), String> {
        let mut first_child = NeuralNetwork::new();
        let mut second_child = NeuralNetwork::new();

        for (first_parent, second_parent) in self
            .neural_network
            .get_layers()
            .iter()
            .zip(other.get_neural_network().get_layers().iter())
        {
            let (c1, c2) = first_parent.crossover(second_parent, randomizer)?;
            first_child.add(c1)?;
            second_child.add(c2)?;
        }

        Ok((Genome::new(first_child), Genome::new(second_child)))
    }
}

impl fmt::Debug for Genome<NeuralNetwork> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--Genome (fitness: {})--", self.fitness)?;
        writeln!(f, "--> Neural network:")?;
        writeln!(f, "{:#?}", self.neural_network)
    }
}

impl<T: NeuralNetworkTrait> GenomeTrait<T> for Genome<T> {
    fn new(neural_network: T) -> Self
    where
        T: NeuralNetworkTrait,
    {
        Genome {
            neural_network,
            fitness: 0f64,
        }
    }

    fn get_neural_network(&self) -> &T
    where
        T: NeuralNetworkTrait,
    {
        &self.neural_network
    }
    fn get_fitness(&self) -> f64 {
        self.fitness
    }
    fn set_fitness(&mut self, fitness: f64) {
        self.fitness = fitness
    }
    fn mutate(&mut self) -> std::result::Result<(), std::string::String> {
        // This only delegates to the neural network.
        self.neural_network.mutate()
    }
}

#[cfg(test)]

mod tests {

    use neural_network::NeuralNetwork;
    extern crate randomization;
    use self::randomization::randomizer::Randomizer;

    use super::*;

    #[test]
    fn test_after_creating_a_genome_it_has_expected_properties() -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let neural_network =
            NeuralNetwork::new_with_specified_layers(&[[4, 3], [3, 2], [2, 1]], &mut randomizer);

        let genome = Genome::<NeuralNetwork>::new(neural_network);

        assert_eq!(genome.get_neural_network().get_number_of_layers(), 3);

        assert_eq!(genome.get_fitness(), 0f64);

        Ok(())
    }
}
