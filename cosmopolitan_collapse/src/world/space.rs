use game_definitions::aspects::Aspects;
use world::space_trait::SpaceTrait;

pub struct Space {
    aspects: Vec<Aspects>,
}

impl SpaceTrait for Space {
    fn new() -> Self {
        Space {
            aspects: Vec::new(),
        }
    }
    fn add_aspect(&mut self, aspect: Aspects) {
        self.aspects.push(aspect);
    }
    fn has_aspect(&self, aspect: Aspects) -> bool {
        self.aspects
            .iter()
            .any(|stored_aspect| stored_aspect == &aspect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_space_can_have_aspects() {
        let mut space = Space::new();

        space.add_aspect(Aspects::River);

        assert_eq!(space.has_aspect(Aspects::River), true);
    }
}
