extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use layer::Layer;
use layer::LayerTrait;

pub trait NeuralNetworkTrait {
    type Layer: LayerTrait;

    fn new() -> Self;
    fn new_with_specified_layers<T: RandomizerTrait>(
        layers_definition: &[[usize; 2]],
        randomizer: &mut T,
    ) -> Self;
    fn get_number_of_layers(&self) -> u32;
    fn get_layer(&self, index: usize) -> &Self::Layer;
    fn add(&mut self, layer: Self::Layer) -> Result<(), String>;
    fn propagate(&self, inputs: &[f64]) -> Result<Vec<f64>, String>;
}

#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetworkTrait for NeuralNetwork {
    type Layer = Layer;

    fn new() -> Self {
        Self { layers: Vec::new() }
    }
    fn get_number_of_layers(&self) -> u32 {
        self.layers.len() as u32
    }
    fn new_with_specified_layers<T: RandomizerTrait>(
        layers_definition: &[[usize; 2]],
        randomizer: &mut T,
    ) -> Self {
        let mut neural_network = NeuralNetwork::new();

        for layer in layers_definition {
            if let Err(error) =
                neural_network.add(Layer::new(layer[0] as u32, layer[1] as u32, randomizer))
            {
                panic!("Failed to add a layer to Neural Network: {:?}", error);
            }
        }

        neural_network
    }
    fn get_layer(&self, index: usize) -> &Self::Layer {
        &self.layers[index]
    }

    fn add(&mut self, layer: Self::Layer) -> std::result::Result<(), std::string::String> {
        if self.layers.is_empty()
            || self.layers.last().unwrap().get_number_of_neurons() == layer.get_number_of_inputs()
        {
            self.layers.push(layer);
            Ok(())
        } else {
            Err("Attempted to add a layer to the neural network, but the number of neurons in the last stored layer didn't match the number of inputs in the new one!".to_string())
        }
    }
    fn propagate(
        &self,
        inputs: &[f64],
    ) -> std::result::Result<std::vec::Vec<f64>, std::string::String> {
        if self.layers.is_empty() {
            panic!("Attempted to propagate inputs through a neural network when there were no layers set up!");
        }
        if self.layers[0].get_number_of_inputs() != inputs.len() as u32 {
            panic!("The number of inputs doesn't match the set inputs in the first layer of the neural network!");
        }

        let mut this_in = inputs;

        let mut this_out: Vec<f64> = Vec::new();

        for layer in &self.layers {
            let temporary = layer.feed_forward(&this_in);

            this_out = temporary;

            this_in = &this_out;
        }

        Ok(this_out)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    use neural_network::randomization::randomizer::Randomizer;

    #[test]
    fn test_when_creating_an_empty_nn_it_has_no_layers() -> Result<(), String> {
        let nn = NeuralNetwork::new();

        assert_eq!(nn.get_number_of_layers(), 0);

        Ok(())
    }

    #[test]
    fn test_when_creating_a_neural_network_with_defined_layers_the_created_neural_networks_layers_have_expected_properties(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let nn =
            NeuralNetwork::new_with_specified_layers(&[[4, 3], [3, 2], [2, 1]], &mut randomizer);
        assert_eq!(nn.get_number_of_layers(), 3);
        assert_eq!(nn.get_layer(0).get_number_of_inputs(), 4);
        assert_eq!(nn.get_layer(0).get_number_of_neurons(), 3);
        assert_eq!(nn.get_layer(2).get_number_of_inputs(), 2);
        assert_eq!(nn.get_layer(2).get_number_of_neurons(), 1);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_creating_a_neural_network_adding_layer_by_layer_should_fail_if_last_layer_to_add_doesnt_connect(
    ) {
        let mut randomizer = Randomizer::new();

        let mut neural_network = NeuralNetwork::new();

        let layer1 = Layer::new(3, 2, &mut randomizer);
        let layer2 = Layer::new(2, 1, &mut randomizer);
        let layer3 = Layer::new(3, 1, &mut randomizer);

        if let Err(error) = neural_network.add(layer1) {
            panic!("Adding the first layer failed: {:?}", error);
        }
        if let Err(error) = neural_network.add(layer2) {
            panic!("Adding the second layer failed: {:?}", error);
        }
        if let Err(error) = neural_network.add(layer3) {
            panic!("Adding the third layer failed: {:?}", error);
        }
    }

    #[test]
    #[should_panic]
    fn test_attempting_to_propagate_inputs_through_neural_network_fails_if_passed_wrong_number_of_inputs(
    ) {
        let mut randomizer = Randomizer::new();

        let neural_network =
            NeuralNetwork::new_with_specified_layers(&[[3, 2], [2, 1]], &mut randomizer);

        let inputs = vec![0.0_f64, 1.0_f64];
        if let Err(error) = neural_network.propagate(&inputs) {
            panic!(format!(
                "Failed when propagating inputs through network: {:?}",
                error
            ));
        }
    }

    #[test]
    fn test_propagating_inputs_through_neural_network_the_produced_output_should_be_in_expected_range(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let neural_network =
            NeuralNetwork::new_with_specified_layers(&[[3, 2], [2, 1]], &mut randomizer);

        let inputs = vec![0.0_f64, 1.0_f64, 0.0_f64];
        let outputs = neural_network.propagate(&inputs)?;

        assert_eq!(outputs.len(), 1);

        assert!(outputs[0] >= 0f64);
        assert!(outputs[0] <= 1f64);

        Ok(())
    }

    #[test]
    fn test_can_lock_down_behavior_of_neural_network() -> Result<(), String> {
        struct FakeRandomizer {}

        impl RandomizerTrait for FakeRandomizer {
            fn get_normal(&mut self) -> f64 {
                0.4_f64
            }
        }

        let mut randomizer = FakeRandomizer {};

        let neural_network =
            NeuralNetwork::new_with_specified_layers(&[[3, 2], [2, 2], [2, 2]], &mut randomizer);

        let inputs = vec![0.0_f64, 1.0_f64, 0.0_f64];
        let outputs = neural_network.propagate(&inputs)?;

        assert_eq!(outputs.len(), 2);

        assert!(outputs[0] >= 0.7265);
        assert!(outputs[0] <= 0.7266);
        assert!(outputs[1] >= 0.7265);
        assert!(outputs[1] <= 0.7266);

        Ok(())
    }
}
