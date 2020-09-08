pub struct IdGenerator {
    last_id: u32,
}

impl Default for IdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator { last_id: 0 }
    }

    pub fn generate(&mut self) -> u32 {
        self.last_id += 1;

        self.last_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_generate_an_id() -> Result<(), String> {
        let mut id_generator = IdGenerator::new();

        assert_eq!(id_generator.generate(), 1);

        Ok(())
    }

    #[test]
    fn test_can_generate_correct_ids_in_succession() -> Result<(), String> {
        let mut id_generator = IdGenerator::new();

        assert_eq!(id_generator.generate(), 1);
        assert_eq!(id_generator.generate(), 2);
        assert_eq!(id_generator.generate(), 3);

        Ok(())
    }
}
