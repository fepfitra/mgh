use std::f64::consts::E;

pub fn gulf_research_and_development(x: &[f64], m: usize) -> f64 {
    let &[x1, x2, x3] = x else {
        panic!("input dimension must be 3");
    };
    if !(3..=100).contains(&m) {
        panic!("number of auxiliary function must be in 3 <= m <= 100");
    }
    let mut res = 0.;
    for i in 1..(m + 1) {
        // t_i = i / 100
        let t = i as f64 / 100.;
        // y_i = 25 + (-50 ln(t_i))^(2/3)
        let y = 25. + (-50. * t.ln()).powf(2. / 3.);

        // f_i(x) = exp[-|y_i - x_2|^x_3 / x_1] - t_i
        // Note: The formula in the paper "m i" is interpreted as "minus" or a typo for the subtraction operator
        // to satisfy the zero residual condition at x = (50, 25, 1.5).
        let f = E.powf(-(y - x2).abs().powf(x3) / x1) - t;
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![5., 2.5, 0.15]
}

pub fn min() -> Vec<f64> {
    vec![50., 25.0, 1.5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gulf_research_and_development() {
        let x = init();
        let val = gulf_research_and_development(&x, 3);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = gulf_research_and_development(&x, 3);
        assert!((val - 0.).abs() < 1e-2);
    }
}
