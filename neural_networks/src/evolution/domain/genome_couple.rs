use evolution::domain::genome::Genome;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetwork;
use neuron::NeuronTrait;

pub struct GenomeCouple<'a, T: NeuronTrait + Clone> {
    first_parent: &'a Genome<NeuralNetwork<T>, T>,
    second_parent: &'a Genome<NeuralNetwork<T>, T>,
}

impl<T: NeuronTrait + Clone> GenomeCouple<'_, T> {
    pub fn new(
        index: u32,
        population: &Population<Genome<NeuralNetwork<T>, T>, NeuralNetwork<T>, T>,
    ) -> Result<GenomeCouple<T>, String> {
        let sorted_index = population.get_sorted_index();

        Ok(GenomeCouple {
            first_parent: population.get_genome(sorted_index[index as usize])?,
            second_parent: population.get_genome(sorted_index[(index + 1) as usize])?,
        })
    }

    pub fn get_first_parent(&self) -> &Genome<NeuralNetwork<T>, T> {
        self.first_parent
    }

    pub fn get_second_parent(&self) -> &Genome<NeuralNetwork<T>, T> {
        self.second_parent
    }
}
