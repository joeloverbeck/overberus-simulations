use evolution::domain::genome::GenomeTrait;
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

impl<
        'a,
        T: GenomeTrait<U, V> + Clone,
        U: NeuralNetworkTrait<V> + Clone,
        V: NeuronTrait + Clone,
    > GenomeCouple<'_, T, U, V>
{
    pub fn new(
        first_parent: &'a T,
        second_parent: &'a T,
    ) -> Result<GenomeCouple<'a, T, U, V>, String> {
        Ok(GenomeCouple {
            first_parent,
            second_parent,
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
