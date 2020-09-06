use agents::agent::Agent;
use agents::agent_trait::AgentTrait;
use agents::brain_trait::BrainTrait;
use agents::decisions::Decisions;

#[derive(Debug)]
pub struct DecisionEntry {
    agent_id: u32,
    decision: Decisions,
}

impl DecisionEntry {
    pub fn new<T: BrainTrait>(agent: &Agent<T>, decision: &Decisions) -> DecisionEntry {
        DecisionEntry {
            agent_id: agent.get_id(),
            decision: decision.clone(),
        }
    }

    pub fn get_agent_id(&self) -> u32 {
        self.agent_id
    }

    pub fn get_decision(&self) -> &Decisions {
        &self.decision
    }
}
