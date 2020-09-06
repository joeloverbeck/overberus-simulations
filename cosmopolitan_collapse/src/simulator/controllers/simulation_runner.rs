use agents::agent::Agent;
use agents::agent_trait::AgentTrait;
use agents::brain_trait::BrainTrait;
use agents::decisions::Decisions;
use game_definitions::aspects::Aspects;
use queries::context_information::ContextInformation;
use simulator::domain::decision_entry::DecisionEntry;
use world::space::Space;
use world::space_trait::SpaceTrait;

pub struct SimulationRunner<T: BrainTrait> {
    spaces: Vec<Space>,
    agents: Vec<Agent<T>>,
    decisions_registry: Vec<DecisionEntry>,
}

impl<T: BrainTrait> SimulationRunner<T> {
    pub fn new(spaces: Vec<Space>) -> SimulationRunner<T> {
        SimulationRunner {
            spaces,
            agents: Vec::new(),
            decisions_registry: Vec::new(),
        }
    }

    pub fn add_agent(&mut self, agent: Agent<T>) {
        self.agents.push(agent);
    }

    pub fn get_agents(&self) -> &Vec<Agent<T>> {
        &self.agents
    }

    pub fn get_spaces(&self) -> &Vec<Space> {
        &self.spaces
    }

    pub fn advance_turn(&mut self) -> Result<(), String> {
        // Every time the turn advances, every agent should decide.
        for agent in self.agents.iter() {
            let context_information = ContextInformation::new();
            let decision = agent.decide(context_information);

            self.decisions_registry
                .push(DecisionEntry::new(&agent, &decision));

            // temporary
            if decision == Decisions::SettleInNaturalShelter {
                self.spaces
                    .iter_mut()
                    .filter(|space| space.get_coordinate() == agent.get_coordinate())
                    .for_each(|space| {
                        space.associate_agent_with_aspect(agent.get_id(), &Aspects::NaturalShelter)
                    });
            }
        }

        Ok(())
    }

    pub fn get_decisions_registry(&self) -> &Vec<DecisionEntry> {
        &self.decisions_registry
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use agents::decisions::Decisions;
    use queries::context_information::ContextInformation;
    use queries::domain::is_agent_associated_with_space_aspect::is_agent_associated_with_space_aspect;
    use world::coordinate::Coordinate;
    use world::space::Space;
    use world::space_trait::SpaceTrait;

    struct FakeBrain {}

    impl BrainTrait for FakeBrain {
        fn decide(&self, _: ContextInformation) -> Decisions {
            Decisions::None
        }
    }

    #[test]
    fn test_can_advance_turn_of_simulation() -> Result<(), String> {
        let spaces: Vec<Space> = vec![
            Space::new(Coordinate::new(0, 0, 0)),
            Space::new(Coordinate::new(1, 0, 0)),
        ];

        let mut simulation_runner = SimulationRunner::<FakeBrain>::new(spaces);

        simulation_runner.add_agent(Agent::new(1, FakeBrain {}, Coordinate::new(0, 0, 0)));
        simulation_runner.add_agent(Agent::new(2, FakeBrain {}, Coordinate::new(0, 0, 0)));

        simulation_runner.advance_turn()?;

        assert_eq!(simulation_runner.get_agents().len(), 2);
        assert_eq!(simulation_runner.get_spaces().len(), 2);

        Ok(())
    }

    #[test]
    fn test_can_get_the_decisions_registry_of_the_simulation_runner() -> Result<(), String> {
        let spaces: Vec<Space> = vec![
            Space::new(Coordinate::new(0, 0, 0)),
            Space::new(Coordinate::new(1, 0, 0)),
        ];

        let mut simulation_runner = SimulationRunner::<FakeBrain>::new(spaces);

        simulation_runner.add_agent(Agent::new(1, FakeBrain {}, Coordinate::new(0, 0, 0)));
        simulation_runner.add_agent(Agent::new(2, FakeBrain {}, Coordinate::new(0, 0, 0)));

        simulation_runner.advance_turn()?;
        simulation_runner.advance_turn()?;

        let decisions_registry = simulation_runner.get_decisions_registry();

        assert_eq!(decisions_registry.len(), 4);
        assert_eq!(decisions_registry[0].get_agent_id(), 1);
        assert_eq!(decisions_registry[1].get_agent_id(), 2);
        assert_eq!(decisions_registry[2].get_agent_id(), 1);
        assert_eq!(decisions_registry[3].get_agent_id(), 2);

        assert_eq!(decisions_registry[0].get_decision(), &Decisions::None);
        assert_eq!(decisions_registry[1].get_decision(), &Decisions::None);
        assert_eq!(decisions_registry[2].get_decision(), &Decisions::None);
        assert_eq!(decisions_registry[3].get_decision(), &Decisions::None);

        Ok(())
    }

    #[test]
    fn test_if_an_agent_decides_to_settle_natural_shelter_it_should_end_up_registering_as_associated_with_that_natural_shelter(
    ) -> Result<(), String> {
        use game_definitions::aspects::Aspects;

        struct BrainThatDecidesToSettleNaturalShelter {}

        impl BrainTrait for BrainThatDecidesToSettleNaturalShelter {
            fn decide(&self, _: ContextInformation) -> Decisions {
                Decisions::SettleInNaturalShelter
            }
        }

        let mut space = Space::new(Coordinate::new(0, 0, 0));

        space.add_aspect(Aspects::NaturalShelter);

        let spaces: Vec<Space> = vec![space];

        let mut simulation_runner = SimulationRunner::new(spaces);

        let agent = Agent::new(
            1,
            BrainThatDecidesToSettleNaturalShelter {},
            Coordinate::new(0, 0, 0),
        );

        simulation_runner.add_agent(agent);

        simulation_runner.advance_turn()?;

        assert_eq!(
            is_agent_associated_with_space_aspect(
                1,
                Coordinate::new(0, 0, 0),
                Aspects::NaturalShelter,
                simulation_runner.get_agents(),
                simulation_runner.get_spaces()
            ),
            true
        );

        Ok(())
    }
}
