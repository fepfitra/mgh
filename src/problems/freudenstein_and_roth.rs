pub fn freudenstein_and_roth(x: &[f64]) -> f64 {
    let &[x1, x2] = x else {
        panic!("input dimension must be 2");
    };

    // f_1(x) = -13 + x1 + ((5 - x2)x2 - 2)x2
    let f1 = -13. + x1 + ((5. - x2) * x2 - 2.) * x2;

    // f_2(x) = -29 + x1 + ((x2 + 1)x2 - 14)x2
    let f2 = -29. + x1 + ((x2 + 1.) * x2 - 14.) * x2;

    f1.powi(2) + f2.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![0.5, -2.]
}

pub fn min() -> Vec<f64> {
    vec![5., 4.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freudenstein_and_roth() {
        let x = init();
        let val = freudenstein_and_roth(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = freudenstein_and_roth(&x);
        assert_eq!(val, 0.);
    }
}
