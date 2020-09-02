use evolution::domain::genome::GenomeTrait;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetworkTrait;
use neuron::NeuronTrait;
use std::marker::PhantomData;

pub struct GenomeCouple<
    'a,
    T: GenomeTrait<U, V> + Clone,
    U: NeuralNetworkTrait<V> + Clone,
    V: NeuronTrait + Clone,
> {
    first_parent: &'a T,
    second_parent: &'a T,
    phantom_u: PhantomData<U>,
    phantom_v: PhantomData<V>,
}

impl<T: GenomeTrait<U, V> + Clone, U: NeuralNetworkTrait<V> + Clone, V: NeuronTrait + Clone>
    GenomeCouple<'_, T, U, V>
{
    pub fn new(
        index: u32,
        population: &Population<T, U, V>,
    ) -> Result<GenomeCouple<T, U, V>, String> {
        let sorted_index = population.get_sorted_index();

        Ok(GenomeCouple {
            first_parent: population.get_genome(sorted_index[index as usize])?,
            second_parent: population.get_genome(sorted_index[(index + 1) as usize])?,
            phantom_u: PhantomData,
            phantom_v: PhantomData,
        })
    }

    pub fn get_first_parent(&self) -> &T {
        self.first_parent
    }

    pub fn get_second_parent(&self) -> &T {
        self.second_parent
    }
}
