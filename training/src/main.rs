extern crate neural_networks;
extern crate randomization;

use self::neural_networks::evolution::domain::population::Population;
use self::randomization::randomizer::Randomizer;
use neural_networks::evolution::controllers::create_next_generation::create_next_generation;

fn main() {
    let mut randomizer = Randomizer::new();

    if let Ok(population) =
        Population::new_with_specified_layers(10, &[[3, 2], [2, 2], [2, 1]], &mut randomizer)
    {
        println!("Created population: {:?}", population);

        let next_generation = create_next_generation(&population, &mut randomizer);

        println!("**Next generation produced**");
        println!("{:?}", next_generation);
    } else {
        println!("Failed to create population.");
    }
}
