use std::f64::consts::E;

pub fn gulf_research_and_development(x: &[f64], m: usize) -> f64 {
    if !(3..=100).contains(&m) {
        panic!("number of auxiliary function must be in 3 <= m <= 100");
    }
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let mut res = 0.;
    for i in 1..(m + 1) {
        let t = i as f64 / 100.;
        let y = 25. + (-50. * t.ln()).powf(2. / 3.);

        let f = E.powf(-(y * m as f64 * i as f64 * x2).abs().powf(x3) / x1) - t;
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
        assert!((val - 0.).abs() < 1e-2); // Use epsilon because of floats
    }
}
