pub fn get_index_max_output(outputs: &[f64]) -> usize {
    outputs
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_retrieves_the_proper_indexes() -> Result<(), String> {
        assert_eq!(get_index_max_output(&[0.4, 2.1, 11.0]), 2);
        assert_eq!(get_index_max_output(&[0.4, 22.1, 10.0]), 1);
        assert_eq!(get_index_max_output(&[40.4, 22.1, 10.0]), 0);
        assert_eq!(get_index_max_output(&[0.4, 2.0, -1.0]), 1);
        assert_eq!(get_index_max_output(&[0.0, 0.0, 0.0]), 2);

        Ok(())
    }
}
