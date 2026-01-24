pub fn wood(x: &[f64]) -> f64 {
    if x.len() != 4 {
        panic!("input dimension must be 4");
    }
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];
    let f1 = 10. * (x2 - x1.powi(2));
    let f2 = 1. - x1;
    let f3 = 90.0_f64.sqrt() * (x4 - x3.powi(2));
    let f4 = 1. - x3;
    let f5 = 10.0_f64.sqrt() * (x2 + x4 - 2.);
    let f6 = 10.0_f64.sqrt().recip() * (x2 - x4);

    f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2) + f5.powi(2) + f6.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![-3., -1., -3., -1.]
}

pub fn min() -> Vec<f64> {
    vec![1., 1., 1., 1.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wood() {
        let x = init();
        let val = wood(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = wood(&x);
        assert_eq!(val, 0.);
    }
}
