pub fn powell_singular(x: &[f64]) -> f64 {
    let &[x1, x2, x3, x4] = x else {
        panic!("input dimension must be 4");
    };

    // f_1(x) = x_1 + 10x_2
    let f1 = x1 + 10. * x2;

    // f_2(x) = 5^0.5(x_3 - x_4)
    let f2 = 5.0_f64.sqrt() * (x3 - x4);

    // f_3(x) = (x_2 - 2x_3)^2
    let f3 = (x2 - 2. * x3).powi(2);

    // f_4(x) = 10^0.5(x_1 - x_4)^2
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
