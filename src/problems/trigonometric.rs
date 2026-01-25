pub fn trigonometric(x: &[f64]) -> f64 {
    let mut res = 0.;
    let sum_cos: f64 = x.iter().map(|xi| xi.cos()).sum();
    let n = x.len() as f64;

    for (i_idx, &xi) in x.iter().enumerate() {
        let i = i_idx as f64 + 1.0;
        // f_i(x) = n - sum(cos x_j) + i(1 - cos x_i) - sin x_i
        let f = n - sum_cos + i * (1. - xi.cos()) - xi.sin();
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![1. / (n as f64); n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric() {
        let n = 3;
        let x = init(n);
        let val = trigonometric(&x);
        assert!(val.is_finite());
    }
}
