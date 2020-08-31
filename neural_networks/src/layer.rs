extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
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
}

pub struct Layer {
    number_of_inputs: u32,
    neurons: Vec<Neuron>,
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
