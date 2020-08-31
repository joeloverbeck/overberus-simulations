extern crate randomization;

use evolution::constants::MUTATION_PROBABILITY;
use self::randomization::randomizer::RandomizerTrait;
use evolution::constants::CROSSOVER_PROBABILITY;
use neuron::Neuron;
use neuron::NeuronTrait;

pub trait LayerTrait {
    fn new<T: RandomizerTrait>(
        number_of_inputs: u32,
        number_of_neurons: u32,
        randomizer: &mut T,
    ) -> Layer;
    fn get_number_of_inputs(&self) -> u32;
    fn get_number_of_neurons(&self) -> u32;
    fn feed_forward(&self, inputs: &[f64]) -> Vec<f64>;
    fn should_crossover<T: RandomizerTrait>(randomizer: &mut T) -> Result<bool, String>;
    fn should_mutate<T: RandomizerTrait>(randomizer: &mut T) -> Result<bool, String>;
}

#[derive(Debug)]
pub struct Layer {
    number_of_inputs: u32,
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn mutate(&self) -> Result<(), String> {
        todo!()
    }

    pub fn crossover<T: RandomizerTrait>(
        &self,
        other: &Layer,
        randomizer: &mut T,
    ) -> Result<(Layer, Layer), String> {
        let mut first_child =
            Layer::new(self.number_of_inputs, self.neurons.len() as u32, randomizer);
        let mut second_child =
            Layer::new(self.number_of_inputs, self.neurons.len() as u32, randomizer);

        // Cannot iter() over here since destructuring assignments are not allowed
        // https://github.com/rust-lang/rfcs/issues/372

        for index in 0..self.neurons.len() as usize {
            if Layer::should_crossover(randomizer)? {
                first_child
                    .get_neuron_mut(index)?
                    .set_bias(other.get_neuron(index)?.get_bias());
                second_child
                    .get_neuron_mut(index)?
                    .set_bias(self.get_neuron(index)?.get_bias());
            } else {
                first_child
                    .get_neuron_mut(index)?
                    .set_bias(self.get_neuron(index)?.get_bias());
                second_child
                    .get_neuron_mut(index)?
                    .set_bias(other.get_neuron(index)?.get_bias());
            }

            for j in 0..self.number_of_inputs as usize {
                if Layer::should_crossover(randomizer)? {
                    first_child
                        .get_neuron_mut(index)?
                        .set_weight(j, other.get_neuron(index)?.get_weight(j)?)?;
                    second_child
                        .get_neuron_mut(index)?
                        .set_weight(j, self.get_neuron(index)?.get_weight(j)?)?;
                } else {
                    first_child
                        .get_neuron_mut(index)?
                        .set_weight(j, self.get_neuron(index)?.get_weight(j)?)?;
                    second_child
                        .get_neuron_mut(index)?
                        .set_weight(j, other.get_neuron(index)?.get_weight(j)?)?;
                }
            }
        }

        Ok((first_child, second_child))
    }

    pub fn get_neuron(&self, index: usize) -> Result<&Neuron, String> {
        Ok(&self.neurons[index])
    }

    pub fn get_neuron_mut(
        &mut self,
        index: usize,
    ) -> std::result::Result<&mut Neuron, std::string::String> {
        Ok(&mut self.neurons[index])
    }
}

impl LayerTrait for Layer {
    fn new<T: RandomizerTrait>(
        number_of_inputs: u32,
        number_of_neurons: u32,
        randomizer: &mut T,
    ) -> Layer {
        Layer {
            number_of_inputs,
            neurons: (0..number_of_neurons)
                .map(|_| Neuron::new(number_of_inputs, randomizer))
                .collect(),
        }
    }

    fn get_number_of_inputs(&self) -> u32 {
        self.number_of_inputs
    }

    fn get_number_of_neurons(&self) -> u32 {
        self.neurons.len() as u32
    }

    fn feed_forward(&self, inputs: &[f64]) -> std::vec::Vec<f64> {
        // Sanity check
        if inputs.len() != self.number_of_inputs as usize {
            panic!(
                "A layer was sent {:?} inputs when it was set up with {:?}",
                inputs.len(),
                self.number_of_inputs
            )
        }

        self.neurons
            .iter()
            .map(|neuron| neuron.activate(&inputs).unwrap())
            .collect()
    }

    fn should_crossover<T>(randomizer: &mut T) -> std::result::Result<bool, std::string::String>
    where
        T: RandomizerTrait,
    {
        Ok(randomizer.generate_f64() > 1f64 - CROSSOVER_PROBABILITY)
    }

    fn should_mutate<T>(randomizer: &mut T) -> std::result::Result<bool, std::string::String> where T: RandomizerTrait { 
        Ok(randomizer.generate_f64() > 1f64 - MUTATION_PROBABILITY)
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use layer::randomization::randomizer::Randomizer;

    fn setup_layer() -> Layer {
        let mut randomizer = Randomizer::new();

        Layer::new(3, 2, &mut randomizer)
    }

    #[test]
    fn test_when_creating_a_layer_it_has_expected_number_of_inputs() -> Result<(), String> {
        let layer = setup_layer();

        assert_eq!(layer.get_number_of_inputs(), 3);

        Ok(())
    }

    #[test]
    fn test_when_creating_a_layer_it_has_expected_number_of_neurons() -> Result<(), String> {
        let layer = setup_layer();

        assert_eq!(layer.get_number_of_neurons(), 2);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_when_feed_forwarding_inputs_through_layer_with_wrong_number_of_inputs_it_should_crash()
    {
        let layer = setup_layer();

        layer.feed_forward(&vec![1f64, 2f64]);
    }

    #[test]
    fn test_when_feed_forwarding_inputs_through_layer_with_right_amount_of_inputs_it_should_produce_right_amount_of_outputs(
    ) -> Result<(), String> {
        let layer = setup_layer();

        let outputs = layer.feed_forward(&vec![0f64, 1f64, 0f64]);

        println!("{:?}", outputs);
        assert_eq!(outputs.len(), 2);

        Ok(())
    }
}
