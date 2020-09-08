use components::controllers::components_controller::ComponentsController;
use components::domain::components::Components;
use components::domain::manipulate_component::manipulate_component;

#[allow(clippy::blocks_in_if_conditions)]
pub fn persist_settle_in_building_decision(
    agent_id: u32,
    space_id: u32,
    components_controller: &mut ComponentsController,
) -> Result<(), String> {
    if !components_controller
        .does_any_component_of_an_entity_check_a_condition(space_id, |component| {
            component.is_building()
        })
    {
        panic!("Was going to persist the decision to settle in a building, but there was no building in space with id {:?}", space_id);
    }

    components_controller.manipulate_component(
        |id, components| {
            id == &space_id && components.iter().any(|component| component.is_building())
        },
        |_id, components| {
            manipulate_component(
                components,
                |component| component.is_building(),
                |component| {
                    if let Components::Building {
                        ref mut inhabitants,
                        ..
                    } = component
                    {
                        inhabitants.push(agent_id);
                    }
                },
            )
            .unwrap();
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use agents::decisions::Decisions;
    use entities::id_generator::IdGenerator;
    use persistence::controllers::memory_persistence_controller::MemoryPersistenceController;

    use super::*;

    #[test]
    fn test_can_settle_agent_in_building() -> Result<(), String> {
        let mut id_generator = IdGenerator::new();

        let agent_id = id_generator.generate();
        let space_id = id_generator.generate();

        let mut components_controller = ComponentsController::new();

        components_controller.add(agent_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;
        components_controller.add(space_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;
        components_controller.add(
            space_id,
            Components::Building {
                inhabitants: Vec::new(),
                room_limit: 1,
            },
        )?;

        let decisions = vec![Decisions::SettleInBuilding {
            agent_id: agent_id,
            space_id: space_id,
        }];

        let memory_persistence_controller = MemoryPersistenceController::new();

        memory_persistence_controller.persist_decisions(&decisions, &mut components_controller)?;

        assert!(
            components_controller.does_any_component_of_an_entity_check_a_condition(
                space_id,
                |component| {
                    if component.is_building() {
                        if let Components::Building {
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
