pub fn variably_dimensional(x: &[f64]) -> f64 {
    let mut res = 0.0;
    for item in x {
        res += (item - 1.).powi(2);
    }
    let fnplus1 = {
        let mut f = 0.0;
        for i in 1..(x.len() + 1) {
            let index = i - 1;
            f += i as f64 * (x[index] - 1.);
        }
        f
    };
    res + fnplus1.powi(2) + fnplus1.powi(4)
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for (i, item) in vec.iter_mut().enumerate().take(n) {
        *item = 1. - (i as f64 + 1.) / (n as f64);
    }
    vec
}

pub fn min(n: usize) -> Vec<f64> {
    vec![1.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variably_dimensional() {
        let n = 10;
        let x = init(n);
        let val = variably_dimensional(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let n = 3;
        let x = min(n);
        let val = variably_dimensional(&x);
        assert_eq!(val, 0.);
    }
}
