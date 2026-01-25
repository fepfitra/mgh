pub fn linear_full_rank(x: &[f64], m: usize) -> f64 {
    if m < x.len() {
        panic!("number of auxiliary function must be at least n");
    }

    let mut res = 0.;
    let sum_x: f64 = x.iter().sum();

    for (_i, &xi) in x.iter().enumerate() {
        // f_i(x) = x_i - (2/m)sum(x_j) - 1
        let f = xi - 2. * sum_x / m as f64 - 1.;
        res += f.powi(2);
    }
    
    // For n < i <= m
    // f_i(x) = -(2/m)sum(x_j) - 1
    for _ in (x.len() + 1)..(m + 1) {
        let f = -2. * sum_x / m as f64 - 1.;
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![1.; n]
}

pub fn min(n: usize) -> Vec<f64> {
    vec![-1.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_full_rank() {
        let n = 5;
        let m = 10;
        let x = init(n);
        let val = linear_full_rank(&x, m);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let n = 5;
        let m = 10;
        let x = min(n);
        let val = linear_full_rank(&x, m);
        // f = m - n = 10 - 5 = 5.
        assert!((val - 5.0).abs() < 1e-8);
    }
}
