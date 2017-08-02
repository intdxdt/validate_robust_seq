extern crate test_float_overlap;

use test_float_overlap::test_overlap;

pub fn validate_sequence(x: &[f64]) -> bool {
    let n = x.len();
    if n < 1 {
        return false;
    }
    for i in 1..n {
        if (x[i - 1]).abs() >= (x[i]).abs() {
            return false;
        }
        if test_overlap(x[i], x[i - 1]) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod validate_seq {
    use super::validate_sequence as validate;

    #[test]
    fn test_validate_seq() {
        assert!( validate(&vec![1e-16, 1.]));
        assert!(!validate(&vec![0.5, 1.5]));
        assert!( validate(&vec![0.]));
        assert!(!validate(&vec![]));
    }
}
