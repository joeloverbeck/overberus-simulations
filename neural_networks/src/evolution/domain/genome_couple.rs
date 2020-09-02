use evolution::domain::genome::Genome;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;

pub struct GenomeCouple<'a, T: NeuralNetworkTrait<U> + Clone, U: NeuronTrait + Clone> {
    first_parent: &'a Genome<T, U>,
    second_parent: &'a Genome<T, U>,
}

impl<T: NeuralNetworkTrait<U> + Clone, U: NeuronTrait + Clone> GenomeCouple<'_, T, U> {
    pub fn new(
        index: u32,
        population: &Population<Genome<T, U>, T, U>,
    ) -> Result<GenomeCouple<T, U>, String> {
        let sorted_index = population.get_sorted_index();

        Ok(GenomeCouple {
            first_parent: population.get_genome(sorted_index[index as usize])?,
            second_parent: population.get_genome(sorted_index[(index + 1) as usize])?,
        })
    }

    pub fn get_first_parent(&self) -> &Genome<T, U> {
        self.first_parent
    }

    pub fn get_second_parent(&self) -> &Genome<T, U> {
        self.second_parent
    }
}
