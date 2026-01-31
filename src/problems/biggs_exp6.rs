use std::f64::consts::E;

pub fn biggs_exp6(x: &[f64], m: usize) -> f64 {
    if m < 6 {
        panic!("number of auxiliary function must be at least 6");
    }
    let &[x1, x2, x3, x4, x5, x6] = x else {
        panic!("input dimension must be 6");
    };

    let mut res = 0.0;
    for i in 1..(m + 1) {
        // t_i = 0.1 * i
        let t = 0.1 * i as f64;

        // y_i = exp(-t) - 5exp(-10t) + 3exp(-4t)
        let y = E.powf(-t) - 5. * E.powf(-10. * t) + 3. * E.powf(-4. * t);

        // f_i(x) = x3 exp(-t x1) - x4 exp(-t x2) + x6 exp(-t x5) - y_i
        let f = x3 * E.powf(-t * x1) - x4 * E.powf(-t * x2) + x6 * E.powf(-t * x5) - y;
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![1., 2., 1., 1., 1., 1.]
}

pub fn min() -> Vec<f64> {
    vec![1., 10., 1., 5., 4., 3.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biggs_exp6() {
        let x = init();
        let val = biggs_exp6(&x, 10);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = biggs_exp6(&x, 6);
        assert_eq!(val, 0.);
    }
}
