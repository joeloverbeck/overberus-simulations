use agents::agent_trait::AgentTrait;
use agents::belief::Belief;
use world::coordinate::Coordinate;

pub struct Agent {
    beliefs: Vec<Belief>,
}

impl AgentTrait for Agent {
    fn new() -> Self {
        Agent {
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
}

#[cfg(test)]
mod tests {

    use super::*;
    use game_definitions::aspects::Aspects;
    use world::coordinate::Coordinate;

    #[test]
    fn test_an_agent_can_have_beliefs() -> Result<(), String> {
        let mut agent = Agent::new();

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
        let mut agent = Agent::new();

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
}
