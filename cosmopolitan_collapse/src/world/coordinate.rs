#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Coordinate {
        Coordinate { x, y, z }
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl Eq for Coordinate {}
