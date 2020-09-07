use agents::belief_types::BeliefTypes;
use components::domain::components::Components;

#[derive(Debug, Clone)]
pub struct Belief {
    coordinate: Components,
    belief: BeliefTypes,
}

impl Belief {
    pub fn new(coordinate: Components, belief: BeliefTypes) -> Result<Belief, String> {
        if !coordinate.is_coordinate() {
            panic!("Attempted to create a belief, but the coordinate passed wasn't a Components::Coordinate: {:?}", coordinate);
        }

        Ok(Belief { coordinate, belief })
    }

    pub fn get_belief(&self) -> &BeliefTypes {
        &self.belief
    }

    pub fn get_coordinate(&self) -> &Components {
        &self.coordinate
    }
}
