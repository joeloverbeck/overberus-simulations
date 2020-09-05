use agents::decisions::Decisions;
use queries::context_information::ContextInformation;

pub trait BrainTrait {
    fn decide(&self, context_information: ContextInformation) -> Decisions;
}
