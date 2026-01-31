use std::f64::consts::E;

pub fn gaussian(x: &[f64]) -> f64 {
    let &[x1, x2, x3] = x else {
        panic!("input dimension must be 3");
    };
    let y = vec![
        0.0009, 0.0044, 0.0175, 0.0540, 0.1295, 0.2420, 0.3521, 0.3989, 0.3521, 0.2420, 0.1295,
        0.0540, 0.0175, 0.0044, 0.0009,
    ];

    let mut res = 0.0;

    for (index, item) in y.iter().enumerate().take(15) {
        let i = index as f64 + 1.;

        // t_i = (8 - i) / 2
        let t = (8. - i) / 2.;

        // f_i(x) = x_1 exp[-x_2(t_i - x_3)^2 / 2] - y_i
        let f = x1 * E.powf((-x2 * (t - x3).powi(2)) / 2.) - item;
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0.4, 1., 0.]
}

pub fn min() -> Vec<f64> {
    vec![0.3989561, 1.0000191, 2.787451e-20]
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

    #[test]
    fn test_global_minimum() {
        let x = min();
        let val = gaussian(&x);
        assert!((val - 1.12793e-8).abs() < 1e-10);
    }
}
