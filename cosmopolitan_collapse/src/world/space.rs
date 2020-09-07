use game_definitions::aspects::Aspects;
use std::collections::HashMap;
use world::aspect_association::AspectAssociation;
use world::coordinate::Coordinate;
use world::space_trait::SpaceTrait;

pub struct Space {
    coordinate: Coordinate,
    aspects: HashMap<Aspects, Vec<AspectAssociation>>,
}

impl SpaceTrait for Space {
    fn new(coordinate: Coordinate) -> Self {
        Space {
            coordinate,
            aspects: HashMap::new(),
        }
    }
    fn add_aspect(&mut self, aspect: Aspects) {
        self.aspects.insert(aspect, vec![AspectAssociation::new()]);
    }
    fn has_aspect(&self, aspect: &Aspects) -> bool {
        self.aspects
            .iter()
            .any(|(stored_aspect, _)| stored_aspect == aspect)
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    fn associate_agent_with_aspect(&mut self, agent_id: u32, aspect: &Aspects) {
        if !self.aspects.contains_key(aspect) {
            panic!("Asked to associate an agent with aspect, but there were no aspects of that kind stored!");
        }

        let corresponding_associations = self.aspects.get_mut(aspect).unwrap();

        if corresponding_associations.is_empty() {
            panic!("There was an entry for aspect {:?} in the space, but there was any entry in its vector for associations!", aspect);
        }

        for association in corresponding_associations.iter_mut() {
            if !association.is_agent_associated(agent_id) {
                association.associate_agent(agent_id);
                return;
            }
        }

        // If we reach this point, something has gone wrong.
        panic!("Attempted to associate agent {:?} with aspect {:?}, but despite finding it in the hashmap of aspects, we couldn't associate the agent to any of its associations!", agent_id, aspect);
    }

    fn is_agent_associated_with_aspect(&self, agent_id: u32, aspect: &Aspects) -> bool {
        if !self.aspects.contains_key(aspect) {
            panic!("Asked whether an agent was associated with an aspect, but there were no aspects of that kind stored!");
        }

        let stored_aspect_associations = self.aspects.get(aspect).unwrap();

        stored_aspect_associations
            .iter()
            .any(|association| association.is_agent_associated(agent_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_space_can_have_aspects() {
        let mut space = Space::new(Coordinate::new(0, 0, 0));

        space.add_aspect(Aspects::River);

        assert_eq!(space.has_aspect(&Aspects::River), true);
    }
}
