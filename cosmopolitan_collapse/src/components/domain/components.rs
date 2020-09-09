extern crate neural_networks;

use self::neural_networks::neural_network::NeuralNetwork;
use self::neural_networks::neuron::Neuron;

use agents::belief::Belief;

#[derive(Debug, Clone, is_enum_variant)]
pub enum Components {
    Coordinate {
        x: i32,
        y: i32,
        z: i32,
    },
    Cave {
        inhabitants: Vec<u32>,
        room_limit: usize,
    },
    Building {
        inhabitants: Vec<u32>,
        room_limit: usize,
    },
    Campament {
        inhabitants: Vec<u32>,
        room_limit: usize,
    },
    Name(String),
    Brain {
        settling_in: NeuralNetwork<Neuron>,
    },
    FakeBrain,
    Beliefs(Vec<Belief>),
}
