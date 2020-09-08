use agents::belief::Belief;

#[derive(Debug, Clone, is_enum_variant)]
pub enum Components {
    Coordinate {
        x: i32,
        y: i32,
        z: i32,
    },
    Cave {
        inhabitants: Vec<u32>,
        room_limit: usize,
    },
    Building {
        inhabitants: Vec<u32>,
        room_limit: usize,
    },
    FakeBrain,
    Beliefs(Vec<Belief>),
}
