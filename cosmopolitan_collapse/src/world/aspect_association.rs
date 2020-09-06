#[derive(Debug)]
pub struct AspectAssociation {
    agents: Vec<u32>,
}

impl Default for AspectAssociation {
    fn default() -> Self {
        Self::new()
    }
}

impl AspectAssociation {
    pub fn new() -> AspectAssociation {
        AspectAssociation { agents: Vec::new() }
    }

    pub fn associate_agent(&mut self, agent_id: u32) {
        self.agents.push(agent_id);
    }

    pub fn is_agent_associated(&self, agent_id: u32) -> bool {
        self.agents.iter().any(|agent| agent == &agent_id)
    }
}
