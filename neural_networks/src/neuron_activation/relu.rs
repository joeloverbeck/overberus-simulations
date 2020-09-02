extern crate float_ord;
use self::float_ord::FloatOrd;

pub fn relu(z: f64) -> f64 {
    std::cmp::max(FloatOrd(0.0), FloatOrd(z)).0
}
