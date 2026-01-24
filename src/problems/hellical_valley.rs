use std::f64::consts::PI;

pub fn hellical_valley(x: &[f64]) -> f64 {
    if x.len() != 3 {
        panic!("input dimension must be 3");
    }

    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];

    let theta = if x1 > 0. {
        (x2 / x1).atan() / (2. * PI)
    } else {
        (x2 / x1).atan() / (2. * PI) + 0.5
    };

    let f1 = 10. * (x3 - 10. * theta);
    let f2 = 10. * ((x1.powi(2) + x2.powi(2)).sqrt() - 1.);
    let f3 = x3;

    f1.powi(2) + f2.powi(2) + f3.powi(2)
}

pub fn init() -> Vec<f64> {
    vec![-1., 0., 0.]
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
}
