use game_definitions::aspects::Aspects;
use world::coordinate::Coordinate;

pub trait SpaceTrait {
    fn new(coordinate: Coordinate) -> Self;
    fn get_coordinate(&self) -> &Coordinate;
    fn add_aspect(&mut self, aspect: Aspects);
    fn has_aspect(&self, aspect: &Aspects) -> bool;
    fn associate_agent_with_aspect(&mut self, agent_id: u32, aspect: &Aspects);
    fn is_agent_associated_with_aspect(&self, agent_id: u32, aspect: &Aspects) -> bool;
}
