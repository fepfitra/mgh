pub fn brown_badly_scaled(x: &[f64]) -> f64 {
    let x1 = x[0];
    let x2 = x[1];

    let f1 = x1 - 1000000.;
    let f2 = x2 - 0.000002;
    let f3 = x1 * x2 - 2.;

    f1.powi(2) + f2.powi(2) + f3.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![1., 1.]
}

pub fn min() -> Vec<f64> {
    vec![1000000., 0.000002]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brown_badly_scaled() {
        let x = init();
        let val = brown_badly_scaled(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = brown_badly_scaled(&x);
        assert_eq!(val, 0.);
    }
}
