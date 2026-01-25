pub fn penalty1(x: &[f64]) -> f64 {
    let a: f64 = 0.00001;
    let mut res = 0.0;
    for item in x {
        // f_i(x) = sqrt(a)(x_i - 1)
        let f = a.sqrt() * (item - 1.);
        res += f.powi(2);
    }
    res += {
        let mut r = 0.;
        for item in x {
            r += item.powi(2);
        }
        // f_{n+1}(x) = (sum x_j^2) - 1/4
        (r - 0.25).powi(2)
    };
    res
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for (i, item) in vec.iter_mut().enumerate().take(n) {
        *item = i as f64 + 1.;
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penalty1() {
        let n = 3;
        let x = init(n);
        let val = penalty1(&x);
        assert!(val.is_finite());
    }
}
