pub fn broyden_banded(x: &[f64]) -> f64 {
    let n = x.len();
    let ml = 5;
    let mu = 1;

    let mut res = 0.0;

    // Loop to calculate each f_i(x).
    for i in 0..n {
        let xi = x[i];

        // f_i(x) = x_i(2 + 5x_i^2) + 1 - sum_{j in J_i} x_j(1 + x_j)
        let first_part = xi * (2.0 + 5.0 * xi.powi(2)) + 1.0;

        let mut sum = 0.0;
        let lower_j = i.saturating_sub(ml);
        let upper_j = (i + mu).min(n - 1);

        for (j, &xj) in x.iter().enumerate().take(upper_j + 1).skip(lower_j) {
            if i != j {
                sum += xj * (1.0 + xj);
            }
        }

        let f = first_part - sum;
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![-1.; n]
}

pub fn min(n: usize) -> Vec<f64> {
    vec![-1.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broyden_banded() {
        let n = 10;
        let x = init(n);
        let val = broyden_banded(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_value_n10() {
        let n = 10;
        let x = init(n);
        let val = broyden_banded(&x);
        // At x=(-1,...), f_i = -6. Sum squares = 36n = 360.
        assert!((val - 360.0).abs() < 1e-8);
    }
}
