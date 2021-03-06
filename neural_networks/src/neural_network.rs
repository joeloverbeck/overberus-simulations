extern crate randomization;
extern crate serde;

use self::randomization::randomizer::RandomizerTrait;
use self::serde::{Deserialize, Serialize};
use layer::Layer;
use layer::LayerTrait;
use neuron::NeuronTrait;

pub trait NeuralNetworkTrait<T: NeuronTrait> {
    fn new_with_specified_layers<U: RandomizerTrait, V: Fn(u32, &mut U) -> T>(
        layers_definition: &[[usize; 2]],
        randomizer: &mut U,
        neuron_creator: V,
    ) -> Self;
    fn get_number_of_layers(&self) -> u32;
    fn get_layer(&self, index: usize) -> &Layer<T>;
    fn get_layers(&self) -> &Vec<Layer<T>>;
    fn get_layers_mut(&mut self) -> &mut Vec<Layer<T>>;
    fn add(&mut self, layer: Layer<T>) -> Result<(), String>;
    fn propagate(&self, inputs: &[f64]) -> Result<Vec<f64>, String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NeuralNetwork<T: NeuronTrait> {
    layers: Vec<Layer<T>>,
}

impl<T: NeuronTrait> NeuralNetwork<T> {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

impl<T: NeuronTrait> Default for NeuralNetwork<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: NeuronTrait> NeuralNetworkTrait<T> for NeuralNetwork<T> {
    fn new_with_specified_layers<U: RandomizerTrait, V: Fn(u32, &mut U) -> T>(
        layers_definition: &[[usize; 2]],
        randomizer: &mut U,
        neuron_creator: V,
    ) -> Self {
        let mut neural_network = NeuralNetwork::new();

        for layer in layers_definition {
            if let Err(error) = neural_network.add(Layer::<T>::create_layer(
                layer[0] as u32,
                layer[1] as u32,
                randomizer,
                &neuron_creator,
            )) {
                panic!("Failed to add a layer to Neural Network: {:?}", error);
            }
        }

        neural_network
    }

    fn get_number_of_layers(&self) -> u32 {
        self.layers.len() as u32
    }

    fn get_layer(&self, index: usize) -> &Layer<T> {
        &self.layers[index]
    }

    fn add(&mut self, layer: Layer<T>) -> std::result::Result<(), std::string::String> {
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
    fn get_layers(&self) -> &Vec<Layer<T>> {
        &self.layers
    }
    fn get_layers_mut(&mut self) -> &mut Vec<Layer<T>> {
        &mut self.layers
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use neural_network::tests::file_system::deserialize_json_from_string::deserialize_json_from_string;
    use neural_network::tests::file_system::read_file_to_string::read_file_to_string;
    use neural_network::tests::file_system::save_json::save_json;
    use neuron::Neuron;
    use neuron_activation::activation_functions::ActivationFunctions;

    use neural_network::randomization::randomizer::Randomizer;

    extern crate file_system;

    #[test]
    fn test_when_creating_an_empty_nn_it_has_no_layers() -> Result<(), String> {
        let nn = NeuralNetwork::<Neuron>::new();

        assert_eq!(nn.get_number_of_layers(), 0);

        Ok(())
    }

    #[test]
    fn test_when_creating_a_neural_network_with_defined_layers_the_created_neural_networks_layers_have_expected_properties(
    ) -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let nn = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[4, 3], [3, 2], [2, 1]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );
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

        let layer1 =
            Layer::<Neuron>::create_layer(3, 2, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            });
        let layer2 =
            Layer::<Neuron>::create_layer(2, 1, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            });
        let layer3 =
            Layer::<Neuron>::create_layer(3, 1, &mut randomizer, |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            });

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

        let neural_network = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[3, 2], [2, 1]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

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

        let neural_network = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[3, 2], [2, 1]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

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

            fn generate_float_from_0_to_1(&mut self) -> f64 {
                todo!()
            }
            fn choose_random_from_vec<T>(&mut self, _: &[T]) -> T
            where
                T: std::clone::Clone,
            {
                todo!()
            }
        }

        let mut randomizer = FakeRandomizer {};

        let neural_network = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[3, 2], [2, 2], [2, 2]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

        let inputs = vec![0.0_f64, 1.0_f64, 0.0_f64];
        let outputs = neural_network.propagate(&inputs)?;

        assert_eq!(outputs.len(), 2);

        assert!(outputs[0] >= 0.7265);
        assert!(outputs[0] <= 0.7266);
        assert!(outputs[1] >= 0.7265);
        assert!(outputs[1] <= 0.7266);

        Ok(())
    }

    #[test]
    fn test_can_serde_a_neural_network() -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let neural_network = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[3, 2], [2, 2], [2, 2]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

        let serialized = serde_json::to_string(&neural_network).unwrap();

        let deserialized: NeuralNetwork<Neuron> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.get_number_of_layers(), 3);

        let first_layer = deserialized.get_layer(0);

        assert_eq!(first_layer.get_number_of_inputs(), 3);
        assert_eq!(first_layer.get_number_of_neurons(), 2);

        let second_layer = deserialized.get_layer(1);

        assert_eq!(second_layer.get_number_of_inputs(), 2);
        assert_eq!(second_layer.get_number_of_neurons(), 2);

        let third_layer = deserialized.get_layer(2);

        assert_eq!(third_layer.get_number_of_inputs(), 2);
        assert_eq!(third_layer.get_number_of_neurons(), 2);

        Ok(())
    }

    #[test]
    fn test_can_save_and_load_a_serialized_neural_network() -> Result<(), String> {
        let mut randomizer = Randomizer::new();

        let neural_network = NeuralNetwork::<Neuron>::new_with_specified_layers(
            &[[3, 2], [2, 2], [2, 2]],
            &mut randomizer,
            |number_of_inputs, randomizer| {
                Neuron::new(number_of_inputs, ActivationFunctions::Sigmoid, randomizer)
            },
        );

        use self::file_system::does_file_exist::does_file_exist;
        use self::file_system::remove_file::remove_file;

        let file_path = "./testdata/neural_network_test.json";

        assert!(
            !does_file_exist(file_path)?,
            "The file path {:?} shouldn't correspond to an existing file",
            file_path
        );

        save_json(file_path, &neural_network)?;

        let file_as_string = read_file_to_string(file_path)?;

        match deserialize_json_from_string::<NeuralNetwork<Neuron>>(&file_as_string) {
            Err(error) => {
                remove_file(file_path)?;
                panic!("Couldn't deserialize {:?}. Error: {:?}", file_path, error);
            }
            Ok(deserialized) => {
                remove_file(file_path)?;

                assert!(!does_file_exist(file_path)?, "After serializing to file and deserializing, the file path {:?} shouldn't correspond to an existing file", file_path);

                assert_eq!(deserialized.get_number_of_layers(), 3);

                let first_layer = deserialized.get_layer(0);

                assert_eq!(first_layer.get_number_of_inputs(), 3);
                assert_eq!(first_layer.get_number_of_neurons(), 2);

                let second_layer = deserialized.get_layer(1);

                assert_eq!(second_layer.get_number_of_inputs(), 2);
                assert_eq!(second_layer.get_number_of_neurons(), 2);

                let third_layer = deserialized.get_layer(2);

                assert_eq!(third_layer.get_number_of_inputs(), 2);
                assert_eq!(third_layer.get_number_of_neurons(), 2);
            }
        }

        Ok(())
    }
}
