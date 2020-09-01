extern crate neural_networks;
extern crate randomization;



use controllers::gym_controller::neural_networks::neuron::NeuronTrait;
use controllers::gym_controller::neural_networks::neural_network::NeuralNetworkTrait;
use controllers::gym_controller::neural_networks::evolution::domain::genome::GenomeTrait;
use self::neural_networks::evolution::domain::population::Population;



pub struct GymController<T: GenomeTrait<U, V> + Clone, U: NeuralNetworkTrait<V> + Clone, V: NeuronTrait + Clone> {
    population: Population<T, U, V>,
}

impl <T: GenomeTrait<U, V> + Clone, U: NeuralNetworkTrait<V> + Clone, V: NeuronTrait + Clone> GymController<T, U, V>{
    pub fn new(population: Population<T, U, V>) -> GymController<T, U, V>{
        GymController{
            population,
        }
    }

    pub fn train(&self) -> Result<Population<T, U, V>, String>{
        Ok(self.population.clone())
    }
}


#[cfg(test)]
mod tests{

use super::*;

#[test]
fn test_can_run_a_training_session_and_receive_trained_population() -> Result<(), String> {
    use self::randomization::randomizer::Randomizer;
    use self::neural_networks::evolution::domain::population::PopulationTrait;

    let mut randomizer = Randomizer::new();

    let population =
        Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer)?;

    let sut = GymController::new(population);

    let trained_population = sut.train()?;

    assert_eq!(trained_population.get_size(), 10);

    Ok(())
}

}