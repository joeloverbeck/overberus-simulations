use agents::belief::Belief;
use world::coordinate::Coordinate;

pub trait AgentTrait {
    fn new() -> Self;
    fn add_belief(&mut self, belief: Belief);
    fn get_beliefs_about_space(&self, coordinate: Coordinate) -> Vec<&Belief>;
}
