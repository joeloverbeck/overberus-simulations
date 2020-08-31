use evolution::domain::genome::Genome;
use evolution::domain::population::Population;
use evolution::domain::population::PopulationTrait;
use neural_network::NeuralNetwork;
use neuron::Neuron;

type NN = NeuralNetwork<Neuron>;
type GN = Genome<NN, Neuron>;

pub struct GenomeCouple<'a> {
    first_parent: &'a GN,
    second_parent: &'a GN,
}

impl GenomeCouple<'_> {
    pub fn new(
        index: u32,
        population: &Population<GN, NN, Neuron>,
    ) -> Result<GenomeCouple, String> {
        let sorted_index = population.get_sorted_index();

        Ok(GenomeCouple {
            first_parent: population.get_genome(sorted_index[index as usize])?,
            second_parent: population.get_genome(sorted_index[(index + 1) as usize])?,
        })
    }

    pub fn get_first_parent(&self) -> &GN {
        self.first_parent
    }

    pub fn get_second_parent(&self) -> &GN {
        self.second_parent
    }
}
