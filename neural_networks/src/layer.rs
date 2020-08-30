extern crate randomization;

use self::randomization::randomizer::RandomizerTrait;
use layer::randomization::randomizer::Randomizer;

pub trait LayerTrait {
    type Randomizer: RandomizerTrait;

    fn new(
        number_of_inputs: u32,
        number_of_neurons: u32,
        randomizer: &mut Self::Randomizer,
    ) -> Layer;
    fn get_number_of_inputs(&self) -> u32;
    fn get_number_of_neurons(&self) -> u32;
    fn get_number_of_weights(&self) -> u32;
    fn get_number_of_biases(&self) -> u32;
}

pub struct Layer {
    number_of_inputs: u32,
    number_of_neurons: u32,
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl Layer {}

impl LayerTrait for Layer {
    type Randomizer = Randomizer;

    fn new(
        number_of_inputs: u32,
        number_of_neurons: u32,
        randomizer: &mut Self::Randomizer,
    ) -> Layer {
        Layer {
            number_of_inputs,
            number_of_neurons,
            biases: (0..number_of_neurons).map(|_| randomizer.get_normal()).collect(),
            weights: (0..number_of_neurons)
                .map(|_| {
                    (0..number_of_inputs)
                        .map(|_| randomizer.get_normal())
                        .collect()
                })
                .collect(),
        }
    }

    fn get_number_of_inputs(&self) -> u32 {
        self.number_of_inputs
    }

    fn get_number_of_neurons(&self) -> u32 {
        self.number_of_neurons
    }

    fn get_number_of_weights(&self) -> u32 {
        self.weights.len() as u32
    }
    fn get_number_of_biases(&self) -> u32 {
        self.biases.len() as u32
    }
}

#[cfg(test)]

mod tests {

    use super::*;

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
    fn test_when_creating_a_layer_it_has_expected_number_of_weights() -> Result<(), String> {
        let layer = setup_layer();

        assert_eq!(layer.get_number_of_weights(), 2);

        Ok(())
    }

    #[test]
    fn test_when_creating_a_layer_it_has_expected_number_of_biases() -> Result<(), String> {
        let layer = setup_layer();

        assert_eq!(layer.get_number_of_biases(), 2);

        Ok(())
    }
}
