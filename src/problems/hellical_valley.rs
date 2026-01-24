use std::f64::consts::PI;

pub fn hellical_valley(x: &[f64]) -> f64 {
    if x.len() != 3 {
        panic!("input dimension must be 3");
    }

    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];

    let theta = if x1 > 0. {
        // theta(x1, x2) = atan(x2 / x1) / 2pi
        (x2 / x1).atan() / (2. * PI)
    } else {
        // theta(x1, x2) = atan(x2 / x1) / 2pi + 0.5
        (x2 / x1).atan() / (2. * PI) + 0.5
    };

    // f_1(x) = 10[x3 - 10theta(x1, x2)]
    let f1 = 10. * (x3 - 10. * theta);

    // f_2(x) = 10[(x1^2 + x2^2)^0.5 - 1]
    let f2 = 10. * ((x1.powi(2) + x2.powi(2)).sqrt() - 1.);

    // f_3(x) = x3
    let f3 = x3;

    f1.powi(2) + f2.powi(2) + f3.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![-1., 0., 0.]
}

pub fn min() -> Vec<f64> {
    vec![1., 0., 0.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hellical_valley() {
        let x = init();
        let val = hellical_valley(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = hellical_valley(&x);
        assert!(val.abs() < 1e-8);
    }
}
