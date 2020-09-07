use agents::belief::Belief;

#[derive(Debug, Clone, is_enum_variant)]
pub enum Components {
    Coordinate { x: i32, y: i32, z: i32 },
    FakeBrain,
    Beliefs(Vec<Belief>),
}

#[cfg(test)]
mod tests {

    use super::*;
    use agents::belief::Belief;
    use agents::decisions::Decisions;

    #[cfg(test)]
    mod tests {
        use super::*;

        use components::controllers::components_controller::ComponentsController;
        use game_definitions::aspects::Aspects;
        use queries::context_information::ContextInformation;
        use world::coordinate::Coordinate;

        #[test]
        fn test_an_agent_can_have_beliefs() -> Result<(), String> {
            let agent_id = 1;

            let mut components_controller = ComponentsController::new();

            components_controller.add(agent_id, Components::FakeBrain)?;
            components_controller.add(agent_id, Components::Coordinate { x: 0, y: -3, z: 3 })?;
            components_controller.add(
                agent_id,
                Components::Beliefs(vec![Belief::new(
                    Components::Coordinate { x: 0, y: -3, z: 3 },
                    Components::River(),
                )?]),
            )?;

            let beliefs_component = components_controller
                .get_components_of_entity(agent_id)?
                .filter(|component| component.is_beliefs())
                .first();

            if let Components::Beliefs(entries) = beliefs_component {
                assert_eq!(
                    entries.iter().any(|entry| entry.get_coordinate().x == 0
                        && entry.get_coordinate().y == -3
                        && entry.get_coordinate().z == 3
                        && entry.get_belief().is_river()),
                    true
                );
            }

            Ok(())
        }

        #[test]
        fn test_if_requesting_beliefs_about_a_place_all_beliefs_should_be_of_that_place(
        ) -> Result<(), String> {
            let agent_id = 1;

            let mut components_controller = ComponentsController::new();

            components_controller.add(agent_id, Components::FakeBrain)?;
            components_controller.add(agent_id, Components::Coordinate { x: 0, y: -3, z: 3 })?;

            components_controller.add(
                agent_id,
                Components::Beliefs(vec![Belief::new(
                    Components::Coordinate { x: 0, y: -3, z: 3 },
                    Components::River(),
                )]),
            )?;

            let beliefs_component = components_controller
                .get_components_of_entity(agent_id)?
                .filter(|component| component.is_beliefs())
                .first()
                .unwrap();

            if let Components::Beliefs(entries) = beliefs_component {
                entries.push(Belief::new(
                    Components::Coordinate { x: -1, y: 1, z: 2 },
                    Components::River(),
                ));

                assert_eq!(
                    entries.iter().any(|entry| entry.get_coordinate().x = 0
                        && entry.get_coordinate().y == -3
                        && entry.get_coordinate().z == 3
                        && entry.get_belief().is_river()),
                    true
                );
                assert_eq!(
                    entries.iter().any(|entry| entry.get_coordinate().x = -1
                        && entry.get_coordinate().y == 1
                        && entry.get_coordinate().z == 2
                        && entry.get_belief().is_river()),
                    true
                );
            }

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
}
