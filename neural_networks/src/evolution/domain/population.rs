extern crate randomization;
extern crate serde;

use self::randomization::randomizer::RandomizerTrait;
use self::serde::{Deserialize, Serialize};
use evolution::domain::genome::Genome;
use evolution::domain::genome::GenomeTrait;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use neuron::Neuron;
use neuron::NeuronTrait;
use std::fmt;
use std::marker::PhantomData;

pub trait PopulationTrait<T: GenomeTrait<U, V>, U: NeuralNetworkTrait<V>, V: NeuronTrait> {
    fn get_size(&self) -> u32;
    fn add(&mut self, genome: T) -> Result<(), String>;
    fn get_genome(&self, index: usize) -> Result<&T, String>;
    fn get_genomes_mut(&mut self) -> Result<&mut Vec<T>, String>;
    fn get_genome_mut(&mut self, index: usize) -> Result<&mut T, String>;
    fn get_sorted_index(&self) -> Vec<usize>;
    fn get_midpoint(&self) -> u32;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Population<
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
> {
    genomes: Vec<T>,
    phantom_u: PhantomData<U>,
    phantom_v: PhantomData<V>,
}

impl fmt::Debug
    for Population<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--Population (size: {})--", self.get_size())?;

        writeln!(f, "--> Genomes:")?;

        for (index, genome) in self.genomes.iter().enumerate() {
            write!(f, "#{:?} {:#?}", index, genome)?;
        }

        writeln!(f)
    }
}

impl<T: GenomeTrait<U, V> + Clone, U: NeuralNetworkTrait<V> + Clone, V: NeuronTrait + Clone> Default
    for Population<T, U, V>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: GenomeTrait<U, V> + Clone, U: NeuralNetworkTrait<V> + Clone, V: NeuronTrait + Clone>
    Population<T, U, V>
{
    pub fn new() -> Self {
        Population::<T, U, V> {
            genomes: Vec::new(),
            phantom_u: PhantomData,
            phantom_v: PhantomData,
        }
    }

    pub fn new_with_specified_layers<W: RandomizerTrait, X: Fn(&[[usize; 2]], &mut W) -> T>(
        number_of_neural_networks: u32,
        layers_definition: &[[usize; 2]],
        genome_creator: X,
        randomizer: &mut W,
    ) -> Result<Self, String> {
        let mut population = Population::new();

        for _ in 0..number_of_neural_networks {
            let genome = genome_creator(layers_definition, randomizer);

            population.add(genome)?;
        }

        Ok(population)
    }
}

impl<T: GenomeTrait<U, V> + Clone, U: NeuralNetworkTrait<V> + Clone, V: NeuronTrait + Clone>
    PopulationTrait<T, U, V> for Population<T, U, V>
{
    fn get_size(&self) -> u32 {
        self.genomes.len() as u32
    }

    fn add(&mut self, genome: T) -> std::result::Result<(), std::string::String> {
        self.genomes.push(genome);

        Ok(())
    }
    fn get_genome(&self, index: usize) -> Result<&T, String> {
        Ok(&self.genomes[index])
    }
    fn get_genome_mut(&mut self, index: usize) -> Result<&mut T, String> {
        Ok(&mut self.genomes[index])
    }
    fn get_sorted_index(&self) -> std::vec::Vec<usize> {
        let mut index: Vec<(usize, f64)> = self
            .genomes
            .iter()
            .enumerate()
            .map(|(index, genome)| (index, genome.get_fitness()))
            .collect();

        index.sort_by(|(_, fitness_a), (_, fitness_b)| {
            fitness_b
                .partial_cmp(fitness_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        index.iter().map(|(index, _)| *index).collect()
    }
    fn get_midpoint(&self) -> u32 {
        self.get_size() % 2 + self.get_size() / 2
    }
    fn get_genomes_mut(
        &mut self,
    ) -> std::result::Result<&mut std::vec::Vec<T>, std::string::String> {
        Ok(&mut self.genomes)
    }
}

#[cfg(test)]
mod tests {

    use self::randomization::randomizer::Randomizer;
    use super::*;
    use evolution::controllers::create_next_generation::create_next_generation;
    use evolution::domain::create_genome::create_genome;
    use layer::Layer;
    use layer::LayerTrait;
    use neuron::Neuron;

    #[test]
    fn test_can_create_empty_population_of_neural_networks() -> Result<(), String> {
        let population = Population::<
            Genome<NeuralNetwork<Neuron>, Neuron>,
            NeuralNetwork<Neuron>,
            Neuron,
        >::new();

        assert_eq!(population.get_size(), 0);

        Ok(())
    }

    #[test]
    fn test_can_create_population_of_x_number_of_neural_networks_with_prespecified_layers(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let layers_definition = &[[4, 3], [3, 2], [2, 1]];

        let population_result = Population::<
            Genome<NeuralNetwork<Neuron>, Neuron>,
            NeuralNetwork<Neuron>,
            Neuron,
        >::new_with_specified_layers(
            10, layers_definition, create_genome, &mut randomizer
        );

        if let Err(error) = population_result {
            panic!(
                "Failed to create a population with specified layers: {:?}",
                error
            );
        }

        let population = population_result.unwrap();

        assert_eq!(population.get_size(), 10);

        Ok(())
    }

    #[test]
    fn test_the_first_neural_network_of_the_population_has_expected_properties(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let layers_definition = &[[4, 3], [3, 2], [2, 1]];

        let population = Population::<
            Genome<NeuralNetwork<Neuron>, Neuron>,
            NeuralNetwork<Neuron>,
            Neuron,
        >::new_with_specified_layers(
            10, layers_definition, create_genome, &mut randomizer
        )?;

        assert_eq!(
            population
                .get_genome(0)?
                .get_neural_network()
                .get_number_of_layers(),
            3
        );
        assert_eq!(
            population
                .get_genome(0)?
                .get_neural_network()
                .get_layer(0)
                .get_number_of_inputs(),
            4
        );
        assert_eq!(
            population
                .get_genome(0)?
                .get_neural_network()
                .get_layer(0)
                .get_number_of_neurons(),
            3
        );

        Ok(())
    }

    #[test]
    fn test_the_last_neural_network_of_the_population_has_expected_properties() -> Result<(), String>
    {
        let mut randomizer = Randomizer::new();

        let layers_definition = &[[4, 3], [3, 2], [2, 1]];

        let population = Population::<
            Genome<NeuralNetwork<Neuron>, Neuron>,
            NeuralNetwork<Neuron>,
            Neuron,
        >::new_with_specified_layers(
            10,
            layers_definition,
            |layers_definition, randomizer| {
                Genome::new(NeuralNetwork::new_with_specified_layers(
                    layers_definition,
                    randomizer,
                    |number_of_inputs, randomizer| Neuron::new(number_of_inputs, randomizer),
                ))
            },
            &mut randomizer,
        )?;

        assert_eq!(
            population
                .get_genome(9)?
                .get_neural_network()
                .get_number_of_layers(),
            3
        );
        assert_eq!(
            population
                .get_genome(9)?
                .get_neural_network()
                .get_layer(2)
                .get_number_of_inputs(),
            2
        );
        assert_eq!(
            population
                .get_genome(9)?
                .get_neural_network()
                .get_layer(2)
                .get_number_of_neurons(),
            1
        );

        Ok(())
    }

    fn setup_manual_population() -> Result<
        Population<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>,
        String,
    > {
        let mut population = Population::new();
        let mut neural_network1 = NeuralNetwork::new();

        let mut randomizer = Randomizer::new();

        let layer1 =
            Layer::<Neuron>::create_layer(3, 2, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, randomizer)
            });
        let layer2 =
            Layer::<Neuron>::create_layer(2, 1, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, randomizer)
            });

        neural_network1.add(layer1)?;
        neural_network1.add(layer2)?;

        population.add(Genome::new(neural_network1))?;

        assert_eq!(population.get_size(), 1);

        let mut neural_network2 = NeuralNetwork::new();

        let layer1 =
            Layer::<Neuron>::create_layer(3, 2, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, randomizer)
            });
        let layer2 =
            Layer::<Neuron>::create_layer(2, 1, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, randomizer)
            });

        neural_network2.add(layer1)?;
        neural_network2.add(layer2)?;

        population.add(Genome::new(neural_network2))?;

        Ok(population)
    }

    #[test]
    fn test_can_add_neural_networks_to_population_one_by_one() -> Result<(), String> {
        let population = setup_manual_population()?;

        assert_eq!(population.get_size(), 2);

        Ok(())
    }

    #[test]
    fn test_can_get_the_sorted_indexes_of_the_population() -> Result<(), String> {
        let mut population = setup_manual_population()?;

        population.get_genome_mut(0)?.set_fitness(5.0_f64);
        population.get_genome_mut(1)?.set_fitness(10.0_f64);

        let sorted_indexes = population.get_sorted_index();

        assert_eq!(sorted_indexes, vec![1, 0]);

        Ok(())
    }

    #[test]
    fn test_after_adding_genomes_to_population_on_the_fly_getting_sorted_index_returns_the_proper_one(
    ) -> Result<(), String> {
        let mut population = setup_manual_population()?;

        population.get_genome_mut(0)?.set_fitness(5.0_f64);
        population.get_genome_mut(1)?.set_fitness(10.0_f64);

        let mut neural_network = NeuralNetwork::new();

        let mut randomizer = Randomizer::new();

        let layer1 =
            Layer::<Neuron>::create_layer(3, 2, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, randomizer)
            });
        let layer2 =
            Layer::<Neuron>::create_layer(2, 1, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, randomizer)
            });

        neural_network.add(layer1)?;
        neural_network.add(layer2)?;

        population.add(Genome::new(neural_network))?;

        population.get_genome_mut(2)?.set_fitness(7.0_f64);

        let sorted_indexes = population.get_sorted_index();

        assert_eq!(sorted_indexes, vec![1, 2, 0]);

        Ok(())
    }

    #[test]
    fn test_can_create_next_generation_from_existing_population() -> Result<(), String> {
        let mut population = setup_manual_population()?;

        population.get_genome_mut(0)?.set_fitness(5.0_f64);
        population.get_genome_mut(1)?.set_fitness(10.0_f64);

        let mut randomizer = Randomizer::new();

        let next_generation = create_next_generation(
            &population,
            Genome::new,
            || NeuralNetwork::new(),
            |number_of_inputs, randomizer| Neuron::new(number_of_inputs, randomizer),
            &mut randomizer,
        )?;

        assert_eq!(next_generation.get_size(), population.get_size());

        Ok(())
    }

    extern crate file_system;

    #[test]
    fn test_can_serialize_population_to_file_and_deserialize_it() -> Result<(), String> {
        use self::file_system::deserialize_json_from_string::deserialize_json_from_string;
        use self::file_system::does_file_exist::does_file_exist;
        use self::file_system::read_file_to_string::read_file_to_string;
        use self::file_system::remove_file::remove_file;
        use self::file_system::save_json::save_json;

        let population = setup_manual_population()?;

        let file_path = "./testdata/population_test.json";

        save_json(file_path, &population)?;

        let file_as_string = read_file_to_string(file_path)?;

        match deserialize_json_from_string::<
            Population<Genome<NeuralNetwork<Neuron>, Neuron>, NeuralNetwork<Neuron>, Neuron>,
        >(&file_as_string)
        {
            Err(error) => {
                remove_file(file_path)?;
                panic!("Couldn't deserialize {:?}. Error: {:?}", file_path, error);
            }

            Ok(deserialized) => {
                remove_file(file_path)?;

                assert!(!does_file_exist(file_path)?, "After serializing to file and deserializing, the file path {:?} shouldn't correspond to an existing file", file_path);

                assert_eq!(deserialized.get_size(), 2);

                let first_genome = deserialized.get_genome(0)?;

                assert_eq!(first_genome.get_neural_network().get_number_of_layers(), 2);

                let second_genome = deserialized.get_genome(1)?;

                assert_eq!(second_genome.get_neural_network().get_number_of_layers(), 2);
            }
        }

        Ok(())
    }
}
