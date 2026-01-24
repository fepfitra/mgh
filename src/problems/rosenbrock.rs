pub fn rosenbrock(x: &[f64]) -> f64 {
    if x.len() != 2 {
        panic!("input dimension must be 2");
    }
    let x1 = x[0];
    let x2 = x[1];

    // f_1(x) = 10(x2 - x1^2)
    let f1 = 10. * (x2 - x1.powi(2));

    // f_2(x) = 1 - x1
    let f2 = 1. - x1;

    f1.powi(2) + f2.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![-1.2, 1.]
}

pub fn min() -> Vec<f64> {
    vec![1., 1.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rosenbrock() {
        let x = init();
        let val = rosenbrock(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = rosenbrock(&x);
        assert_eq!(val, 0.);
    }
}
