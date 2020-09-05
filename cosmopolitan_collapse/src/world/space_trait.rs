use game_definitions::aspects::Aspects;

pub trait SpaceTrait {
    fn new() -> Self;
    fn add_aspect(&mut self, aspect: Aspects);
    fn has_aspect(&self, aspect: Aspects) -> bool;
}
