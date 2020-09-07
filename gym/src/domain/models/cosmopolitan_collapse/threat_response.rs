extern crate cosmopolitan_collapse;
extern crate neural_networks;
extern crate randomization;

#[cfg(test)]
mod tests {

    use domain::models::cosmopolitan_collapse::threat_response::cosmopolitan_collapse::world::coordinate::Coordinate;
use domain::models::cosmopolitan_collapse::threat_response::cosmopolitan_collapse::agents::decisions::Decisions;
use domain::models::cosmopolitan_collapse::threat_response::cosmopolitan_collapse::queries::context_information::ContextInformation;
use domain::models::cosmopolitan_collapse::threat_response::cosmopolitan_collapse::agents::brain_trait::BrainTrait;
use domain::models::cosmopolitan_collapse::threat_response::cosmopolitan_collapse::agents::agent::Agent;
    use domain::models::cosmopolitan_collapse::threat_response::cosmopolitan_collapse::agents::agent_trait::AgentTrait;

    use super::*;

    use self::neural_networks::evolution::domain::genome::Genome;
    use self::neural_networks::evolution::domain::genome::GenomeTrait;
    use self::neural_networks::evolution::domain::population::Population;
    use self::neural_networks::get_index_max_output::get_index_max_output;
    use self::neural_networks::neural_network::NeuralNetwork;
    use self::neural_networks::neural_network::NeuralNetworkTrait;
    use self::neural_networks::neuron::Neuron;
    use self::neural_networks::neuron::NeuronTrait;
    use self::neural_networks::neuron_activation::choose_random_activation_function::choose_random_activation_function;
    use self::randomization::randomizer::Randomizer;
    use controllers::gym_controller::GymController;

    #[test]
    fn test_can_train_at_least_an_agent_to_always_flee() -> Result<(), String> {
        let number_of_neural_networks = 20;
        let layers_definition = &[[1, 4], [4, 4], [4, 3]];

        let mut randomizer = Randomizer::new();

        let training_population = Population::new_with_specified_layers(
            number_of_neural_networks,
            layers_definition,
            |genome_identifier, layers_definition, randomizer| {
                Genome::new(
                    genome_identifier,
                    NeuralNetwork::new_with_specified_layers(
                        layers_definition,
                        randomizer,
                        |number_of_inputs: u32, randomizer: &mut Randomizer| {
                            Neuron::new(
                                number_of_inputs,
                                choose_random_activation_function(randomizer),
                                randomizer,
                            )
                        },
                    ),
                )
            },
            &mut randomizer,
        )?;

        let continue_condition =
            |generation_number| if generation_number <= 10 { true } else { false };

        struct TestingBrain<'a> {
            neural_network: &'a NeuralNetwork<Neuron>,
        }

        impl<'a> TestingBrain<'a> {
            pub fn new(neural_network: &NeuralNetwork<Neuron>) -> TestingBrain {
                TestingBrain { neural_network }
            }
        }

        impl<'a> BrainTrait for TestingBrain<'a> {
            fn decide(&self, context_information: ContextInformation) -> Decisions {
                // Here we test the neural network.
                let outputs = self
                    .neural_network
                    .propagate(&[context_information.get_threat_level_of_current_space()])
                    .unwrap();

                // Outputs should have three.
                if outputs.len() > 3 {
                    panic!(
                        "The experiment expected only three outputs! Outputs: {:?}",
                        outputs
                    );
                }

                if get_index_max_output(&outputs) == 0 {
                    Decisions::Flee
                } else {
                    Decisions::None
                }
            }
        }

        let train_genomes = |genomes: &mut Vec<Genome<NeuralNetwork<Neuron>, Neuron>>,
                             _randomizer: &mut Randomizer| {
            // An agent should be created, slotted the neural network as a brain,
            // passed an aspect of threat, and produce three outputs.
            // We consider that the first output should be the highest, corresponding to flee
            // in our current testing experiment.
            for genome in genomes.iter_mut() {
                let testing_brain = TestingBrain::new(genome.get_neural_network());

                let agent = Agent::new(1, testing_brain, Coordinate::new(0, 0, 0));

                let context_information = ContextInformation::new();

                let decision = agent.decide(context_information);

                if decision == Decisions::Flee {
                    genome.set_fitness(10.0);
                }
            }

            Ok(())
        };

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
                |genome_identifier, neural_network| Genome::new(genome_identifier, neural_network),
                || NeuralNetwork::new(),
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

        // Now test again to see if indeed it has done as expected.
        assert_eq!(gym.get_winner().get_fitness(), 10.0);

        Ok(())
    }
}
