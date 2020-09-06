use agents::agent_trait::AgentTrait;
use agents::belief::Belief;
use agents::brain_trait::BrainTrait;
use agents::decisions::Decisions;
use queries::context_information::ContextInformation;
use world::coordinate::Coordinate;

pub struct Agent<T: BrainTrait> {
    id: u32,
    brain: T,
    coordinate: Coordinate,
    beliefs: Vec<Belief>,
}

impl<T: BrainTrait> AgentTrait<T> for Agent<T> {
    fn new(id: u32, brain: T, coordinate: Coordinate) -> Self {
        Agent {
            id,
            brain,
            coordinate,
            beliefs: Vec::new(),
        }
    }
    fn add_belief(&mut self, belief: Belief) {
        self.beliefs.push(belief);
    }
    fn get_beliefs_about_space(&self, coordinate: Coordinate) -> std::vec::Vec<&Belief> {
        self.beliefs
            .iter()
            .filter(|belief| belief.get_coordinate() == &coordinate)
            .collect::<Vec<_>>()
    }

    fn decide(&self, context_information: ContextInformation) -> Decisions {
        self.brain.decide(context_information)
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use game_definitions::aspects::Aspects;
    use queries::context_information::ContextInformation;
    use world::coordinate::Coordinate;

    struct FakeBrain {}

    impl BrainTrait for FakeBrain {
        fn decide(&self, _: ContextInformation) -> Decisions {
            Decisions::None
        }
    }

    #[test]
    fn test_an_agent_can_have_beliefs() -> Result<(), String> {
        let mut agent = Agent::new(1, FakeBrain {}, Coordinate::new(0, -3, 3));

        agent.add_belief(Belief::new(Coordinate::new(0, -3, 3), Aspects::River));

        assert_eq!(
            agent
                .get_beliefs_about_space(Coordinate::new(0, -3, 3))
                .iter()
                .any(|belief| belief.get_aspect() == &Aspects::River),
            true
        );

        Ok(())
    }

    #[test]
    fn test_if_requesting_beliefs_about_a_place_all_beliefs_should_be_of_that_place(
    ) -> Result<(), String> {
        let mut agent = Agent::new(1, FakeBrain {}, Coordinate::new(0, -3, 3));

        agent.add_belief(Belief::new(Coordinate::new(0, -3, 3), Aspects::River));
        agent.add_belief(Belief::new(Coordinate::new(-1, 1, 2), Aspects::River));

        assert_eq!(
            agent
                .get_beliefs_about_space(Coordinate::new(0, -3, 3))
                .iter()
                .all(|belief| belief.get_coordinate() == &Coordinate::new(0, -3, 3)),
            true
        );

        Ok(())
    }

    #[test]
    fn test_can_ask_an_agent_to_make_a_decision() -> Result<(), String> {
        let agent = Agent::new(1, FakeBrain {}, Coordinate::new(0, -3, 3));

        let decision = agent.decide(ContextInformation::new());

        assert_eq!(decision, Decisions::None);

        Ok(())
    }

    #[test]
    fn test_agent_is_in_a_space() -> Result<(), String> {
        let agent = Agent::new(1, FakeBrain {}, Coordinate::new(0, -3, 3));

        assert_eq!(agent.get_coordinate(), &Coordinate::new(0, -3, 3));

        Ok(())
    }
}
