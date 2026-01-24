pub fn powell_singular(x: &[f64]) -> f64 {
    if x.len() != 4 {
        panic!("input dimension must be 4");
    }

    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];

    let f1 = x1 + 10. * x2;
    let f2 = 5.0_f64.sqrt() * (x3 - x4);
    let f3 = (x2 - 2. * x3).powi(2);
    let f4 = 10.0_f64.sqrt() * (x1 - x4).powi(2);

    f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![3., -1., 0., 1.]
}

pub fn min() -> Vec<f64> {
    vec![0., 0., 0., 0.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powell_singular() {
        let x = init();
        let val = powell_singular(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = powell_singular(&x);
        assert_eq!(val, 0.);
    }
}
