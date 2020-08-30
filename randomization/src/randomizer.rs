extern crate rand;
extern crate rand_distr;

use self::rand::{rngs::ThreadRng, Rng};
use self::rand_distr::{Distribution, Normal};

pub trait RandomizerTrait {
    fn get_normal(&mut self) -> f64;
}

pub struct Randomizer {
    rng: ThreadRng,
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
}
