pub fn discrete_boundary_value(x: &[f64]) -> f64 {
    let mut res = 0.;
    for i in 1..(x.len() + 1) {
        let index = i - 1;
        let h = 1. / (x.len() as f64 + 1.);
        let t = i as f64 * h;

        let plus1 = if index + 1 < x.len() {
            x[index + 1]
        } else {
            0.
        };
        let minus1 = if index > 0 { x[index - 1] } else { 0. };

        let f = 2. * x[index] - minus1 - plus1 + h.powi(2) * (x[index] + t + 1.).powi(3) / 2.;

        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    let h = 1. / (n as f64 + 1.);
    let mut vec = vec![0.; n];
    for i in 0..n {
        let t = (i as f64 + 1.) * h;
        vec[i] = t * (t - 1.);
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrete_boundary_value() {
        let n = 3;
        let x = init(n);
        let val = discrete_boundary_value(&x);
        assert!(val.is_finite());
    }
}
