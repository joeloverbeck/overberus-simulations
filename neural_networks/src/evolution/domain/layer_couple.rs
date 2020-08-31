use layer::Layer;
use neuron::NeuronTrait;

pub struct LayerCouple<'a, T: NeuronTrait> {
    first_parent: &'a Layer<T>,
    second_parent: &'a Layer<T>,
}

impl<T: NeuronTrait> LayerCouple<'_, T> {
    pub fn new<'a>(
        first_parent: &'a Layer<T>,
        second_parent: &'a Layer<T>,
    ) -> Result<LayerCouple<'a, T>, String> {
        Ok(LayerCouple {
            first_parent,
            second_parent,
        })
    }

    pub fn get_first_parent(&self) -> &Layer<T> {
        self.first_parent
    }

    pub fn get_second_parent(&self) -> &Layer<T> {
        self.second_parent
    }
}
