use agents::belief::Belief;

#[derive(Debug, Clone, is_enum_variant)]
pub enum Components {
    Coordinate { x: i32, y: i32, z: i32 },
    FakeBrain,
    Beliefs(Vec<Belief>),
}
