use std::f64::consts::E;

pub fn gaussian(x: &[f64]) -> f64 {
    if x.len() != 3 {
        panic!("input dimension must be 3");
    }
    let y = vec![
        0.0009, 0.0044, 0.0175, 0.0540, 0.1295, 0.2420, 0.3521, 0.3989, 0.3521, 0.2420, 0.1295,
        0.0540, 0.0175, 0.0044, 0.0009,
    ];
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];

    let mut res = 0.0;

    for index in 0..15 {
        let i = index as f64 + 1.;
        let t = (8. - i) / 2.;
        let f = x1 * E.powf((-x2 * (t - x3).powi(2)) / 2.) - y[index];
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0.4, 1., 0.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaussian() {
        let x = init();
        let val = gaussian(&x);
        assert!(val.is_finite());
    }
}
