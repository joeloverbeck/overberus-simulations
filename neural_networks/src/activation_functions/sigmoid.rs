pub fn sigmoid(z: f64) -> f64 {
    let e = std::f64::consts::E;
    1.0 / (1.0 + e.powf(-z))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sigmoid() {
        assert!(sigmoid(-1000f64) >= 0f64);
        assert!(sigmoid(-1000f64) <= 0.5f64);
        assert!(sigmoid(1000f64) >= 0.5f64);
        assert!(sigmoid(1000f64) <= 1f64);
        assert_eq!(sigmoid(0f64), 0.5f64);
    }
}
