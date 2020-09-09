use agents::decisions::Decisions;
use components::controllers::components_controller::ComponentsController;
use persistence::domain::persist_setting_up_camp_decision::persist_setting_up_camp_decision;
use persistence::domain::persist_settle_in_building_decision::persist_settle_in_building_decision;
use persistence::domain::persist_settle_in_cave_decision::persist_settle_in_cave_decision;

pub struct MemoryPersistenceController {}

impl Default for MemoryPersistenceController {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryPersistenceController {
    pub fn new() -> MemoryPersistenceController {
        MemoryPersistenceController {}
    }

    pub fn persist_decisions(
        &self,
        decisions: &[Decisions],
        components_controller: &mut ComponentsController,
    ) -> Result<(), String> {
        // Should go through the decisions one by one and modify the appropriate components depending on the circumstances it finds the involved
        // components in.
        decisions.iter().for_each(|decision| match decision {
            Decisions::SettleInCave { agent_id, space_id } => {
                persist_settle_in_cave_decision(*agent_id, *space_id, components_controller)
                    .unwrap()
            }
            Decisions::SettleInBuilding { agent_id, space_id } => {
                persist_settle_in_building_decision(*agent_id, *space_id, components_controller)
                    .unwrap()
            }
            Decisions::SetUpCamp { agent_id, space_id } => {
                persist_setting_up_camp_decision(*agent_id, *space_id, components_controller)
                    .unwrap()
            }
            _ => panic!(
                "Persist decisions not handled for decision type {:?}",
                decision
            ),
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use components::controllers::components_controller::ComponentsController;
    use components::domain::components::Components;
    use entities::id_generator::IdGenerator;

    use super::*;

    #[test]
    fn test_can_persist_a_batch_of_decisions() -> Result<(), String> {
        let mut id_generator = IdGenerator::new();

        let agent_id = id_generator.generate();
        let space_id = id_generator.generate();

        let memory_persistence_controller = MemoryPersistenceController::new();

        let mut components_controller = ComponentsController::new();

        components_controller.add(agent_id, Components::Coordinate { x: 1, y: 2, z: 3 })?;
        components_controller.add(space_id, Components::Coordinate { x: 1, y: 2, z: 3 })?;
        components_controller.add(
            space_id,
            Components::Cave {
                inhabitants: Vec::new(),
                room_limit: 1,
            },
        )?;

        let decisions: Vec<Decisions> = vec![Decisions::SettleInCave {
            agent_id: agent_id,
            space_id: space_id,
        }];

        memory_persistence_controller.persist_decisions(&decisions, &mut components_controller)?;

        let matching_cave = components_controller
            .get_component_of_entity(space_id, |owned_component| owned_component.is_cave())?;

        assert_eq!(matching_cave.is_cave(), true);

        if let Components::Cave { inhabitants, .. } = matching_cave {
            assert_eq!(inhabitants.iter().any(|id| id == &agent_id), true);
        }

        Ok(())
    }
}
