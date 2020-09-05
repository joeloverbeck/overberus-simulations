use game_definitions::aspects::Aspects;
use world::coordinate::Coordinate;

#[derive(Clone)]
pub struct Belief {
    coordinate: Coordinate,
    aspect: Aspects,
}

impl Belief {
    pub fn new(coordinate: Coordinate, aspect: Aspects) -> Belief {
        Belief { coordinate, aspect }
    }

    pub fn get_aspect(&self) -> &Aspects {
        &self.aspect
    }

    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }
}
