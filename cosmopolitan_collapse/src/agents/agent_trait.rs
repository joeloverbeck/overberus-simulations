use agents::belief::Belief;
use agents::brain_trait::BrainTrait;
use agents::decisions::Decisions;
use queries::context_information::ContextInformation;
use world::coordinate::Coordinate;

pub trait AgentTrait<T: BrainTrait> {
    fn new(brain: T) -> Self;
    fn add_belief(&mut self, belief: Belief);
    fn get_beliefs_about_space(&self, coordinate: Coordinate) -> Vec<&Belief>;
    fn decide(&self, context_information: ContextInformation) -> Decisions;
}
