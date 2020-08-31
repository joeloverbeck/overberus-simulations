extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::population::Population;
use self::randomization::randomizer::Randomizer;

fn main() {
    let mut randomizer = Randomizer::new();

    if let Ok(population) =
        Population::new_with_specified_layers(10, &[[3, 2], [2, 2], [2, 1]], &mut randomizer)
    {
        println!("Created population: {:?}", population);
    } else {
        println!("Failed to create population.");
    }
}
