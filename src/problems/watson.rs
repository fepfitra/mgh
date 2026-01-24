pub fn watson(x: &[f64]) -> f64 {
    if x.len() < 2 || x.len() > 31 {
        panic!("input dimension must be 2<=n<=31");
    }
    let mut res = 0.0;
    for i in 1..30 {
        let t = i as f64 / 29.;
        let l1 = {
            let mut r = 0.0;
            for j in 2..(x.len() + 1) {
                let jndex = j - 1;
                r += (j as f64 - 1.) * x[jndex] * t.powi(j as i32 - 2);
            }
            r
        };
        let l2 = {
            let mut r = 0.0;
            for j in 1..(x.len() + 1) {
                let jndex = j - 1;
                r += (x[jndex] * t.powi(j as i32 - 1)).powi(2)
            }
            r.powi(2)
        };
        res += (l1 - l2 - 1.).powi(2);
    }
    let x1 = x[0];
    let x2 = x[1];
    res += x1.powi(2);
    res += (x2 - x1.powi(2) - 1.).powi(2);
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![0.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watson() {
        let n = 3;
        let x = init(n);
        let val = watson(&x);
        assert!(val.is_finite());
    }
}
