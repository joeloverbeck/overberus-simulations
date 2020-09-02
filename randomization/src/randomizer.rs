extern crate rand;
extern crate rand_distr;

use self::rand::prelude::SliceRandom;
use self::rand::rngs::ThreadRng;
use self::rand::Rng;
use self::rand_distr::{Distribution, Normal};

pub trait RandomizerTrait {
    fn get_normal(&mut self) -> f64;
    fn generate_float_from_0_to_1(&mut self) -> f64;
    fn choose_random_from_vec<T: Clone>(&mut self, vector: &[T]) -> T;
}

pub struct Randomizer {
    rng: ThreadRng,
}

impl Default for Randomizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Randomizer {
    pub fn new() -> Randomizer {
        Randomizer {
            rng: rand::thread_rng(),
        }
    }
}

impl RandomizerTrait for Randomizer {
    fn get_normal(&mut self) -> f64 {
        Normal::new(0.0, 1.0).unwrap().sample(&mut self.rng)
    }

    fn generate_float_from_0_to_1(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    fn choose_random_from_vec<T: Clone>(&mut self, vector: &[T]) -> T {
        vector.choose(&mut self.rng).unwrap().clone()
    }
}
