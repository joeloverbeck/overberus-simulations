extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::evolution::domain::population::PopulationTrait;
use controllers::gym_controller::neural_networks::evolution::controllers::create_next_generation::create_next_generation;
use controllers::gym_controller::neural_networks::evolution::domain::genome::GenomeTrait;
use controllers::gym_controller::neural_networks::neural_network::NeuralNetworkTrait;
use controllers::gym_controller::neural_networks::neuron::NeuronTrait;
use controllers::gym_controller::randomization::randomizer::RandomizerTrait;

pub struct GymController<
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
    W: Fn(u32) -> bool,
    X: Fn(&mut T) -> Result<(), String>,
> {
    population: Population<T, U, V>,
    generations: u32,
    continue_condition: W,
    train_genome: X,
}

impl<
        T: GenomeTrait<U, V> + Clone + std::fmt::Debug,
        U: NeuralNetworkTrait<V> + Clone,
        V: NeuronTrait + Clone,
        W: Fn(u32) -> bool,
        X: Fn(&mut T) -> Result<(), String>,
    > GymController<T, U, V, W, X>
{
    pub fn new(
        population: Population<T, U, V>,
        continue_condition: W,
        train_genome: X,
    ) -> GymController<T, U, V, W, X> {
        GymController {
            population,
            generations: 0,
            continue_condition,
            train_genome,
        }
    }

    pub fn train<Y: RandomizerTrait, Z: Fn(U) -> T, A: Fn() -> U, B: Fn(u32, &mut Y) -> V>(
        &mut self,
        genome_creator: Z,
        neural_network_creator: A,
        neuron_creator: B,
        randomizer: &mut Y,
    ) -> Result<Population<T, U, V>, String> {
        while (self.continue_condition)(self.generations) {
            for genome in self.population.get_genomes_mut()? {
                (self.train_genome)(genome)?;
            }

            self.population = create_next_generation(
                &self.population,
                &genome_creator,
                &neural_network_creator,
                &neuron_creator,
                randomizer,
            )?;

            println!(
                "Generation {:?} result: {:?}",
                self.generations, self.population
            );

            self.generations += 1;
        }

        Ok(self.population.clone())
    }

    pub fn get_generations(&self) -> u32 {
        self.generations
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use controllers::gym_controller::neural_networks::evolution::domain::genome::Genome;
    use controllers::gym_controller::neural_networks::neural_network::NeuralNetwork;
    use controllers::gym_controller::neural_networks::neuron::Neuron;

    use self::neural_networks::evolution::domain::create_genome::create_genome;

    #[test]
    fn test_can_run_a_training_session_and_receive_trained_population() -> Result<(), String> {
        use self::neural_networks::evolution::domain::population::PopulationTrait;
        use self::randomization::randomizer::Randomizer;

        let mut randomizer = Randomizer::new();

        let layers_definition = &[[4, 3], [3, 2], [2, 1]];

        let population = Population::new_with_specified_layers(
            10,
            layers_definition,
            create_genome,
            &mut randomizer,
        )?;

        let mut sut = GymController::new(
            population,
            |generations: u32| {
                if generations < 10 {
                    true
                } else {
                    false
                }
            },
            |genome| -> Result<(), String> {
                println!("Training genome: {:?}", genome);
                Ok(())
            },
        );

        let mut randomizer = Randomizer::new();

        let trained_population = sut.train(
            |neural_network| Genome::new(neural_network),
            || NeuralNetwork::new(),
            |number_of_inputs, randomizer| Neuron::new(number_of_inputs, randomizer),
            &mut randomizer,
        )?;

        assert_eq!(trained_population.get_size(), 10);
        assert_eq!(sut.get_generations(), 10);

        Ok(())
    }
}
