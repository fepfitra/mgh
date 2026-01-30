pub fn variably_dimensioned(x: &[f64]) -> f64 {
    let mut res = 0.0;
    for item in x {
        // f_i(x) = x_i - 1
        res += (item - 1.).powi(2);
    }
    
    let sum_j = {
        let mut f = 0.0;
        for (i, &xi) in x.iter().enumerate() {
            let j = i as f64 + 1.0;
            f += j * (xi - 1.);
        }
        f
    };
    
    // f_{n+1}(x) = sum j(x_j - 1)
    // f_{n+2}(x) = (sum j(x_j - 1))^2
    // F(x) = sum f_i^2 + f_{n+1}^2 + f_{n+2}^2
    //      = sum (x_i-1)^2 + sum_j^2 + sum_j^4
    res + sum_j.powi(2) + sum_j.powi(4)
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
    fn test_variably_dimensioned() {
        let n = 10;
        let x = init(n);
        let val = variably_dimensioned(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let n = 3;
        let x = min(n);
        let val = variably_dimensioned(&x);
        assert_eq!(val, 0.);
    }
}