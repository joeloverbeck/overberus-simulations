use components::domain::components::Components;
use std::collections::HashMap;

pub struct ComponentsController {
    components: HashMap<u32, Vec<Components>>,
}

impl ComponentsController {
    pub fn new() -> ComponentsController {
        ComponentsController {
            components: HashMap::new(),
        }
    }

    pub fn get_component_of_entity<T: Fn(&&Components) -> bool>(
        &self,
        entity_id: u32,
        condition: T,
    ) -> Result<&Components, String> {
        if !self.components.contains_key(&entity_id) {
            panic!("Attempted to retrieve a component of an entity, but there were no entries for entity {:?}.", entity_id);
        }

        let matched_components: Vec<&Components> = self
            .components
            .get(&entity_id)
            .unwrap()
            .iter()
            .filter(condition)
            .collect();

        if matched_components.len() > 1 {
            panic!("Attempted to retrieve a component of an entity, but it had more than one component matching the passed condition. Retrieved: {:?}", matched_components);
        }

        Ok(matched_components.first().unwrap())
    }

    pub fn does_any_component_of_an_entity_check_a_condition<T: Fn(&Components) -> bool>(
        &self,
        entity_id: u32,
        condition: T,
    ) -> bool {
        if !self.components.contains_key(&entity_id) {
            return false;
        }

        self.components
            .get(&entity_id)
            .unwrap()
            .iter()
            .any(condition)
    }

    pub fn add(&mut self, entity_id: u32, component: Components) -> Result<(), String> {
        self.components
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(component);

        Ok(())
    }

    pub fn get_entity_id_matching_condition<T: Fn(&Vec<Components>) -> bool>(
        &self,
        condition: T,
    ) -> Result<u32, String> {
        let matching_ids = self
            .components
            .iter()
            .filter(|(_, components)| condition(components))
            .map(|(id, _)| id)
            .collect::<Vec<&u32>>();

        if matching_ids.is_empty() {
            panic!("Attempted to get the entity id matching a condition, but there was none!");
        }

        if matching_ids.len() > 1 {
            panic!("Attempted to get the entity id matching a condition, but there was more than one! Matching: {:?}", matching_ids);
        }

        Ok(**matching_ids.first().unwrap())
    }
}

impl Default for ComponentsController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use entities::id_generator::IdGenerator;

    #[test]
    fn can_insert_a_component_for_an_entity() -> Result<(), String> {
        let mut components_controller = ComponentsController::new();

        components_controller.add(1, Components::FakeBrain)?;

        assert_eq!(
            components_controller
                .does_any_component_of_an_entity_check_a_condition(1, |owned_component| {
                    owned_component.is_fake_brain()
                }),
            true
        );

        Ok(())
    }

    #[test]
    fn can_retrieve_values_of_a_stored_component() -> Result<(), String> {
        let mut components_controller = ComponentsController::new();

        let mut id_generator = IdGenerator::new();

        let entity_id = id_generator.generate();

        components_controller.add(entity_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;

        let stored_coordinate = components_controller
            .get_component_of_entity(entity_id, |owned_component| {
                owned_component.is_coordinate()
            })?;

        match stored_coordinate {
            Components::Coordinate { x: 0, y: -3, z: 2 } => assert!(true),
            _ => assert!(false),
        }

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_attempting_to_retrieve_a_component_when_the_entity_has_more_than_one_of_them_it_will_fail(
    ) {
        let mut components_controller = ComponentsController::new();

        let mut id_generator = IdGenerator::new();

        let entity_id = id_generator.generate();

        if let Err(error) =
            components_controller.add(entity_id, Components::Coordinate { x: 0, y: -3, z: 2 })
        {
            panic!("Failed to add component. Error: {:?}", error);
        }

        if let Err(error) =
            components_controller.add(entity_id, Components::Coordinate { x: 0, y: -4, z: 2 })
        {
            panic!("Failed to add component. Error: {:?}", error);
        }

        if let Err(error) = components_controller
            .get_component_of_entity(entity_id, |owned_component| owned_component.is_coordinate())
        {
            panic!("There were two components matching. Error: {:?}", error);
        }
    }

    #[test]
    fn test_can_add_a_cave_component_for_a_space_entity() -> Result<(), String> {
        let mut components_controller = ComponentsController::new();

        let mut id_generator = IdGenerator::new();

        let space_id = id_generator.generate();
        let agent_id = id_generator.generate();

        components_controller.add(space_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;
        components_controller.add(agent_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;

        components_controller.add(
            space_id,
            Components::Cave {
                inhabitants: Vec::new(),
                room_limit: 1,
            },
        )?;

        let matching_space_id =
            components_controller.get_entity_id_matching_condition(|components| {
                components.iter().any(|component| match component {
                    Components::Coordinate { x: 0, y: -3, z: 2 } => true,
                    _ => false,
                }) && components.iter().any(|component| component.is_cave())
            })?;

        let matching_cave = components_controller
            .get_component_of_entity(matching_space_id, |owned_component| {
                owned_component.is_cave()
            })?;

        assert!(matching_cave.is_cave());

        Ok(())
    }

    #[test]
    fn test_can_add_a_building_for_a_space_entity() -> Result<(), String> {
        let mut components_controller = ComponentsController::new();

        let mut id_generator = IdGenerator::new();

        let space_id = id_generator.generate();

        components_controller.add(space_id, Components::Coordinate { x: 0, y: -3, z: 2 })?;
        components_controller.add(
            space_id,
            Components::Building {
                inhabitants: Vec::new(),
                room_limit: 1,
            },
        )?;

        assert_eq!(
            components_controller
                .does_any_component_of_an_entity_check_a_condition(space_id, |component| component
                    .is_building()),
            true
        );

        Ok(())
    }
}
