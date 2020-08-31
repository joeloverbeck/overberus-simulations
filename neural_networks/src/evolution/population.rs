extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use evolution::genome::Genome;
use evolution::genome::GenomeTrait;
use neural_network::NeuralNetwork;
use neural_network::NeuralNetworkTrait;
use std::fmt;
use std::marker::PhantomData;

pub trait PopulationTrait<T: GenomeTrait<U>, U: NeuralNetworkTrait> {
    fn get_size(&self) -> u32;
    fn add(&mut self, genome: T) -> Result<(), String>;
    fn get_genome(&self, index: usize) -> Result<&T, String>;
    fn get_genome_mut(&mut self, index: usize) -> Result<&mut T, String>;
    fn get_sorted_index(&self) -> Vec<usize>;
}

pub struct Population<T: GenomeTrait<U>, U: NeuralNetworkTrait> {
    genomes: Vec<T>,
    phantom: PhantomData<U>,
}

impl fmt::Debug for Population<Genome<NeuralNetwork>, NeuralNetwork> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--Population (size: {})--", self.get_size())?;

        writeln!(f, "--> Genomes:")?;

        for genome in &self.genomes {
            write!(f, "{:#?}", genome)?;
        }

        writeln!(f)
    }
}

impl Default for Population<Genome<NeuralNetwork>, NeuralNetwork> {
    fn default() -> Self {
        Self::new()
    }
}

impl Population<Genome<NeuralNetwork>, NeuralNetwork> {
    pub fn new() -> Self {
        Population::<Genome<NeuralNetwork>, NeuralNetwork> {
            genomes: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn new_with_specified_layers<T: RandomizerTrait>(
        number_of_neural_networks: u32,
        layer_definition: &[[usize; 2]],
        randomizer: &mut T,
    ) -> Result<Self, String>
    where
        Self: std::marker::Sized,
    {
        let mut population = Population::new();

        for _ in 0..number_of_neural_networks {
            let genome = Genome::new(NeuralNetwork::new_with_specified_layers(
                layer_definition,
                randomizer,
            ));

            population.add(genome)?;
        }

        Ok(population)
    }
}

impl<T: GenomeTrait<U>, U: NeuralNetworkTrait> PopulationTrait<T, U> for Population<T, U> {
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
}

#[cfg(test)]
mod tests {

    use self::randomization::randomizer::Randomizer;
    use super::*;
    use layer::Layer;
    use layer::LayerTrait;

    #[test]
    fn test_can_create_empty_population_of_neural_networks() -> Result<(), String> {
        let population = Population::new();

        assert_eq!(population.get_size(), 0);

        Ok(())
    }

    #[test]
    fn test_can_create_population_of_x_number_of_neural_networks_with_prespecified_layers(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let population_result =
            Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer);

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

        let population =
            Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer)?;

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

        let population =
            Population::new_with_specified_layers(10, &[[4, 3], [3, 2], [2, 1]], &mut randomizer)?;

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

    fn setup_manual_population() -> Result<Population<Genome<NeuralNetwork>, NeuralNetwork>, String>
    {
        let mut population = Population::new();
        let mut neural_network1 = NeuralNetwork::new();

        let mut randomizer = Randomizer::new();

        let layer1 = Layer::new(3, 2, &mut randomizer);
        let layer2 = Layer::new(2, 1, &mut randomizer);

        neural_network1.add(layer1)?;
        neural_network1.add(layer2)?;

        population.add(Genome::new(neural_network1))?;

        assert_eq!(population.get_size(), 1);

        let mut neural_network2 = NeuralNetwork::new();

        let layer1 = Layer::new(3, 2, &mut randomizer);
        let layer2 = Layer::new(2, 1, &mut randomizer);

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

        let layer1 = Layer::new(3, 2, &mut randomizer);
        let layer2 = Layer::new(2, 1, &mut randomizer);

        neural_network.add(layer1)?;
        neural_network.add(layer2)?;

        population.add(Genome::new(neural_network))?;

        population.get_genome_mut(2)?.set_fitness(7.0_f64);

        let sorted_indexes = population.get_sorted_index();

        assert_eq!(sorted_indexes, vec![1, 2, 0]);

        Ok(())
    }
}
