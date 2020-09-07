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

    pub fn get_components_of_entity(&self, entity_id: u32) -> Result<&Vec<Components>, String> {
        Ok(&self.components.get(&entity_id).unwrap())
    }

    pub fn add(&mut self, entity_id: u32, component: Components) -> Result<(), String> {
        self.components
            .entry(entity_id)
            .or_insert_with(Vec::new)
            .push(component);

        Ok(())
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

    #[test]
    pub fn can_insert_a_component_for_an_entity() -> Result<(), String> {
        let mut components_controller = ComponentsController::new();

        components_controller.add(1, Components::FakeBrain)?;

        assert_eq!(
            components_controller
                .get_components_of_entity(1)?
                .iter()
                .any(|owned_component| owned_component.is_fake_brain()),
            true
        );

        Ok(())
    }
}
