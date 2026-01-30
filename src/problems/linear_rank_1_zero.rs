pub fn linear_rank_1_zero(x: &[f64], m: usize) -> f64 {
    let n = x.len();
    if m < n {
        panic!("number of auxiliary function must be at least n");
    }

    let mut res = 0.;
    
    // Calculate S = sum_{j=2}^{n-1} j * x_j
    let mut s = 0.0;
    if n > 2 {
        for (j, &xj) in x.iter().enumerate().take(n - 1).skip(1) {
            let math_j = (j + 1) as f64;
            s += math_j * xj;
        }
    }

    // f_1(x) = -1
    res += (-1.0_f64).powi(2);

    // f_i(x) for 2 <= i < m
    for i in 2..m {
        let f = (i as f64 - 1.0) * s - 1.0;
        res += f.powi(2);
    }

    // f_m(x) = -1
    res += (-1.0_f64).powi(2);

    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![1.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_rank_1_zero() {
        let n = 5;
        let m = 10;
        let x = init(n);
        let val = linear_rank_1_zero(&x, m);
        assert!(val.is_finite());
    }

    #[test]
    fn test_optimal_value() {
        // f = (m^2 + 3m - 6) / 2(2m - 3)
        // condition: sum(j*xj) from j=2 to n-1 = 3 / (2m - 3)
        let n = 5;
        let m = 10;
        
        // Target Sum = 3 / (20 - 3) = 3 / 17
        let target_sum = 3.0 / (2.0 * m as f64 - 3.0);
        
        // Construct x satisfying sum_{j=2}^{n-1} j*x_j = target_sum
        // Range j=2 to n-1 involves indices 1 to n-2.
        // For n=5, indices 1, 2, 3. (x[1], x[2], x[3]).
        // Let x[1] (j=2) contribute everything.
        // 2 * x[1] = target_sum => x[1] = target_sum / 2.
        let mut x = vec![0.0; n];
        x[1] = target_sum / 2.0;

        let val = linear_rank_1_zero(&x, m);
        
        let m_f = m as f64;
        let expected = (m_f.powi(2) + 3.0 * m_f - 6.0) / (2.0 * (2.0 * m_f - 3.0));
        
        assert!((val - expected).abs() < 1e-8);
    }
}
