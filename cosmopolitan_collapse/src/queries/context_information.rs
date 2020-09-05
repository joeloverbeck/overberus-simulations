pub struct ContextInformation {}

impl Default for ContextInformation {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextInformation {
    pub fn new() -> ContextInformation {
        ContextInformation {}
    }

    pub fn get_threat_level_of_current_space(&self) -> f64 {
        1.0
    }
}
