pub fn penalty1(x: &[f64]) -> f64 {
    let a: f64 = 0.00001;
    let mut res = 0.0;
    for index in 0..x.len() {
        let f = a.sqrt() * (x[index] - 1.);
        res += f.powi(2);
    }
    res += {
        let mut r = 0.;
        for i in 0..x.len() {
            r += x[i].powi(2);
        }
        (r - 0.25).powi(2)
    };
    res
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for i in 0..n {
        vec[i] = i as f64 + 1.;
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
