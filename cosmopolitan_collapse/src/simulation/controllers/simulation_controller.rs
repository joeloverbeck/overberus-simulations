use agents::decisions::Decisions;
use components::controllers::components_controller::ComponentsController;
use entities::id_generator::IdGenerator;
use persistence::controllers::memory_persistence_controller::MemoryPersistenceController;
use std::collections::VecDeque;

pub struct SimulationController {
    id_generator: IdGenerator,
    memory_persistence_controller: MemoryPersistenceController,
    components_controller: ComponentsController,
    decisions_registry: VecDeque<Decisions>,
}

impl Default for SimulationController {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationController {
    pub fn new() -> SimulationController {
        SimulationController {
            id_generator: IdGenerator::new(),
            memory_persistence_controller: MemoryPersistenceController::new(),
            components_controller: ComponentsController::new(),
            decisions_registry: VecDeque::new(),
        }
    }

    fn produce_decisions(&self) -> Vec<Decisions> {
        self.components_controller
            .get_components()
            .iter()
            .filter(|(_id, components)| {
                components.iter().any(|component| component.is_fake_brain())
            })
            .map(|(_id, components)| {
                components
                    .iter()
                    .filter(|component| component.is_fake_brain())
                    .map(|_component| Decisions::None)
                    .collect::<Vec<Decisions>>()
                    .first()
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<Decisions>>()
    }

    pub fn step(&mut self) -> Result<(), String> {
        // It goes through a single turn of the simulation.

        // Every entity that has a brain gets to decide what to do.
        // Note: given that I use enum "generics" now, I can't rely on a trait to make the brain
        // decide, and I have to handle the stubbed ones as well.
        let decisions = self.produce_decisions();

        self.decisions_registry.extend(decisions.clone());

        self.memory_persistence_controller
            .persist_decisions(&decisions, &mut self.components_controller)?;

        Ok(())
    }

    pub fn get_decisions_registry(&self) -> &VecDeque<Decisions> {
        &self.decisions_registry
    }

    pub fn add_entity<T: Fn(u32, &mut ComponentsController)>(
        &mut self,
        entity_builder: T,
    ) -> Result<(), String> {
        entity_builder(
            self.id_generator.generate(),
            &mut self.components_controller,
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use components::domain::components::Components;

    #[test]
    fn test_can_run_the_simulation() -> Result<(), String> {
        let mut simulation_controller = SimulationController::new();

        simulation_controller.add_entity(|id, components_controller| {
            components_controller
                .add(id, Components::Coordinate { x: 0, y: -3, z: 2 })
                .unwrap();
        })?;

        simulation_controller.add_entity(|id, components_controller| {
            components_controller
                .add(id, Components::Coordinate { x: 1, y: -3, z: 3 })
                .unwrap();
            components_controller
                .add(
                    id,
                    Components::Cave {
                        inhabitants: Vec::new(),
                        room_limit: 1,
                    },
                )
                .unwrap();
        })?;

        simulation_controller.step()?;

        Ok(())
    }

    #[test]
    fn test_can_retrieve_the_decisions_registry_from_the_simulation() -> Result<(), String> {
        let mut simulation_controller = SimulationController::new();

        simulation_controller.add_entity(|id, components_controller| {
            components_controller
                .add(id, Components::FakeBrain)
                .unwrap();
        })?;

        simulation_controller.add_entity(|id, components_controller| {
            components_controller
                .add(id, Components::FakeBrain)
                .unwrap();
        })?;

        simulation_controller.add_entity(|id, components_controller| {
            components_controller
                .add(id, Components::Coordinate { x: 1, y: -3, z: 3 })
                .unwrap();
            components_controller
                .add(
                    id,
                    Components::Cave {
                        inhabitants: Vec::new(),
                        room_limit: 1,
                    },
                )
                .unwrap();
        })?;

        simulation_controller.step()?;

        let decisions = simulation_controller.get_decisions_registry();

        assert_eq!(decisions.len(), 2);

        Ok(())
    }
}
