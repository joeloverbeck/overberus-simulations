extern crate serde;

use self::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ActivationFunctions {
    Sigmoid,
    Relu,
    Softplus,
    Sinusoid,
    Tanh,
    Cosine,
    Swish,
}
