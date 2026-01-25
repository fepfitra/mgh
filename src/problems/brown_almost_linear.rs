pub fn brown_almost_linear(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let prod_x: f64 = x.iter().product();
    let mut res = 0.0;

    // For 1 <= i < n (indices 0 to n-2)
    // f_i(x) = x_i + sum(x_j) - (n + 1)
    for xi in x.iter().take(x.len() - 1) {
        let f = xi + sum_x - (n + 1.0);
        res += f.powi(2);
    }

    // f_n(x) = prod(x_j) - 1
    let fn_val = prod_x - 1.0;
    res += fn_val.powi(2);

    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![0.5; n]
}

pub fn min(n: usize) -> Vec<f64> {
    // There are multiple minima, one is when alpha = 1 -> (1, ..., 1)
    // yielding f=0.
    vec![1.0; n] 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brown_almost_linear() {
        let n = 10;
        let x = init(n);
        let val = brown_almost_linear(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let n = 5;
        let x = min(n); // (1, ..., 1)
        let val = brown_almost_linear(&x);
        assert_eq!(val, 0.);
    }

    #[test]
    fn test_secondary_solution() {
        // f=1 at (0, ..., 0, n+1)
        let n = 5;
        let mut x = vec![0.0; n];
        x[n-1] = (n as f64) + 1.0;
        let val = brown_almost_linear(&x);
        // f_i = 0 + (n+1) - (n+1) = 0 for i < n
        // f_n = 0 - 1 = -1
        // sum = 0 + (-1)^2 = 1.
        assert!((val - 1.0).abs() < 1e-8);
    }
}
