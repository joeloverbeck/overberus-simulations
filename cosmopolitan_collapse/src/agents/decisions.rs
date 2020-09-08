#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Decisions {
    None,
    SettleInCave { agent_id: u32, space_id: u32 },
    SettleInBuilding,
    SetUpCamp,
}
