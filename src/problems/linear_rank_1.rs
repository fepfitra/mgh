pub fn linear_rank_1(x: &[f64], m: usize) -> f64 {
    if m < x.len() {
        panic!("number of auxiliary function must be at least n");
    }

    let mut res = 0.;
    
    // S = sum_{j=1}^n j x_j
    let mut s = 0.0;
    for (j, &xj) in x.iter().enumerate() {
        s += (j as f64 + 1.0) * xj;
    }

    // f_i(x) = i S - 1
    for i in 1..=m {
        let f = (i as f64) * s - 1.0;
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![1.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_rank_1() {
        let n = 5;
        let m = 10;
        let x = init(n);
        let val = linear_rank_1(&x, m);
        assert!(val.is_finite());
    }

    #[test]
    fn test_optimal_value() {
        // f = m(m-1) / 2(2m+1) when sum(j*xj) = 3 / (2m+1)
        let n = 5;
        let m = 10;
        let target_sum = 3.0 / (2.0 * m as f64 + 1.0);
        
        // Construct x such that sum(j*xj) = target_sum
        // Let x_1 = target_sum, resulting sum = 1 * target_sum.
        let mut x = vec![0.0; n];
        x[0] = target_sum;

        let val = linear_rank_1(&x, m);
        let expected = (m as f64 * (m as f64 - 1.0)) / (2.0 * (2.0 * m as f64 + 1.0));
        
        assert!((val - expected).abs() < 1e-8);
    }
}
