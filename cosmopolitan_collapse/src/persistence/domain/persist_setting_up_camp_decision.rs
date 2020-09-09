use components::controllers::components_controller::ComponentsController;
use components::domain::components::Components;

#[allow(clippy::blocks_in_if_conditions)]
pub fn persist_setting_up_camp_decision(
    agent_id: u32,
    space_id: u32,
    components_controller: &mut ComponentsController,
) -> Result<(), String> {
    // Rules for setting up camp: there isn't already a camp in the space. It must be built.
    // Then the agent_id gets added to the list of inhabitants.

    components_controller.add(
        space_id,
        Components::Campament {
            inhabitants: vec![agent_id],
            room_limit: 1,
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use agents::decisions::Decisions;
    use entities::id_generator::IdGenerator;
    use persistence::controllers::memory_persistence_controller::MemoryPersistenceController;

    #[test]
    fn test_can_settle_in_camp() -> Result<(), String> {
        let mut id_generator = IdGenerator::new();

        let agent_id = id_generator.generate();
        let space_id = id_generator.generate();

        let mut components_controller = ComponentsController::new();

        components_controller.add(agent_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;
        components_controller.add(space_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;
        components_controller.add(
            space_id,
            Components::Campament {
                inhabitants: Vec::new(),
                room_limit: 1,
            },
        )?;

        let decisions = vec![Decisions::SetUpCamp {
            agent_id: agent_id,
            space_id: space_id,
        }];

        MemoryPersistenceController::new()
            .persist_decisions(&decisions, &mut components_controller)?;

        assert!(
            components_controller.does_any_component_of_an_entity_check_a_condition(
                space_id,
                |component| {
                    if component.is_campament() {
                        if let Components::Campament {
                            ref inhabitants, ..
                        } = component
                        {
                            if inhabitants.iter().any(|id| id == &agent_id) {
                                return true;
                            }
                        }
                    }

                    return false;
                }
            )
        );

        Ok(())
    }
}
