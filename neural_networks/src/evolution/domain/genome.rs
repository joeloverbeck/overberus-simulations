use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use std::fmt;

pub trait GenomeTrait<T: NeuralNetworkTrait> {
    fn new(neural_network: T) -> Self;
    fn get_neural_network(&self) -> &T;
    fn get_neural_network_mut(&mut self) -> &mut T;
    fn get_fitness(&self) -> f64;
    fn set_fitness(&mut self, fitness: f64);
}

pub struct Genome<T: NeuralNetworkTrait> {
    neural_network: T,
    fitness: f64,
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

    fn get_neural_network_mut(&mut self) -> &mut T {
        &mut self.neural_network
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
