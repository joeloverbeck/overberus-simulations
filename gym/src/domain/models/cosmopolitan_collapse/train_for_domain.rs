extern crate cosmopolitan_collapse;
extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::genome::Genome;
use self::neural_networks::evolution::domain::population::Population;
use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neuron::Neuron;
use self::neural_networks::neuron::NeuronTrait;
use self::neural_networks::neuron_activation::choose_random_activation_function::choose_random_activation_function;
use self::randomization::randomizer::Randomizer;
use controllers::gym_controller::GymController;
use domain::create_standard_training_population::create_standard_training_population;

pub fn train_for_domain<
    T: Fn(u32, &Option<Genome<NeuralNetwork<Neuron>, Neuron>>) -> bool,
    U: Fn(&mut Vec<Genome<NeuralNetwork<Neuron>, Neuron>>, &mut Randomizer) -> Result<(), String>,
>(
    layers_definition: &[[usize; 2]],
    continue_condition: T,
    train_genomes: U,
) -> Result<Genome<NeuralNetwork<Neuron>, Neuron>, String> {
    let number_of_neural_networks = 20;

    let mut randomizer = Randomizer::new();

    let training_population = create_standard_training_population(
        number_of_neural_networks,
        layers_definition,
        &mut randomizer,
    )?;

    let operation_to_perform_on_evolved_population =
        |_evolved_population: &Population<
            Genome<NeuralNetwork<Neuron>, Neuron>,
            NeuralNetwork<Neuron>,
            Neuron,
        >,
         _randomizer: &mut Randomizer| Ok(());

    let mut gym = GymController::new(
        training_population,
        continue_condition,
        train_genomes,
        operation_to_perform_on_evolved_population,
    );

    let _ = gym
        .train(
            Genome::new,
            NeuralNetwork::new,
            |number_of_inputs, randomizer| {
                Neuron::new(
                    number_of_inputs,
                    choose_random_activation_function(randomizer),
                    randomizer,
                )
            },
            |_generation_number, _population| {},
            &mut randomizer,
        )
        .unwrap();

    Ok(gym.get_winner().clone())
}

#[cfg(test)]
mod tests {

    use super::*;

    use self::neural_networks::evolution::domain::genome::GenomeTrait;
    use self::neural_networks::get_index_max_output::get_index_max_output;
    use self::neural_networks::neural_network::NeuralNetworkTrait;

    #[test]
    fn test_can_train_urban_neural_network_for_settling_in_component() -> Result<(), String> {
        let winner = train_for_domain(
            &[[3, 4], [4, 4], [4, 3]],
            |generation_number: u32,
             current_winner: &Option<Genome<NeuralNetwork<Neuron>, Neuron>>| {
                if generation_number <= 10 || current_winner.as_ref().unwrap().get_fitness() < 20.0
                {
                    true
                } else {
                    false
                }
            },
            |genomes: &mut Vec<Genome<NeuralNetwork<Neuron>, Neuron>>,
             _randomizer: &mut Randomizer|
             -> Result<(), String> {
                for genome in genomes.iter_mut() {
                    // For this domain:
                    // Inputs: [0] CavesPresent [1] BuildingsPresent [2] IsHomeless
                    // Outputs: [0] SettleInCave [1] SettleInBuilding [2] SetUpCamp

                    let outputs = genome
                        .get_neural_network()
                        .propagate(&[1.0, 1.0, 1.0])
                        .unwrap();

                    if get_index_max_output(&outputs) == 1 {
                        let current_fitness = genome.get_fitness();
                        genome.set_fitness(current_fitness + 10.0);
                    }

                    let outputs = genome
                        .get_neural_network()
                        .propagate(&[0.0, 1.0, 1.0])
                        .unwrap();

                    if get_index_max_output(&outputs) == 1 {
                        let current_fitness = genome.get_fitness();
                        genome.set_fitness(current_fitness + 10.0);
                    }
                }

                Ok(())
            },
        )?;

        assert_eq!(winner.get_fitness(), 20.0);

        Ok(())
    }
}
