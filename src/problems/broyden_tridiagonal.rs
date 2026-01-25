pub fn broyden_tridiagonal(x: &[f64]) -> f64 {
    let mut new_x = vec![0.];
    new_x.extend_from_slice(x);
    new_x.push(0.);

    let mut res = 0.;
    for i in 1..(new_x.len() - 1) {
        // f_i(x) = (3 - 2x_i)x_i - x_{i-1} - 2x_{i+1} + 1
        let f = (3. - 2. * new_x[i]) * new_x[i] - new_x[i - 1] - 2. * new_x[i + 1] + 1.;
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![-1.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broyden_tridiagonal() {
        let n = 10;
        let x = init(n);
        assert_eq!(x.len(), n);
        assert_eq!(x[0], -1.0);
        let val = broyden_tridiagonal(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_value_n10() {
        let n = 10;
        let x = init(n);
        let val = broyden_tridiagonal(&x);
        // At x=(-1,...), f = n + 11 = 21
        assert!((val - 21.0).abs() < 1e-8);
    }
}
