use std::f64::consts::E;

pub fn powell_badly_scaled(x: &[f64]) -> f64 {
    if x.len() != 2 {
        panic!("input dimension must be 2");
    }
    let x1 = x[0];
    let x2 = x[1];

    // f_1(x) = 10^4 * x1 * x2 - 1
    let f1 = 10000. * x1 * x2 - 1.;

    // f_2(x) = exp(-x1) + exp(-x2) - 1.0001
    let f2 = E.powf(-x1) + E.powf(-x2) - 1.0001;

    f1.powi(2) + f2.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![0., 1.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powell_badly_scaled() {
        let x = init();
        let val = powell_badly_scaled(&x);
        assert!(val.is_finite());
    }
}
