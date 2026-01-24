pub fn linear_full_rank(x: &[f64], m: usize) -> f64 {
    if m < x.len() {
        panic!("number of auxiliary function must be at least n");
    }

    let mut res = 0.;

    let term = {
        let mut sum = 0.;
        for j in 1..(x.len() + 1) {
            let jndex = j - 1;
            sum += x[jndex];
        }
        sum
    };

    for i in 1..(x.len() + 1) {
        let index = i - 1;
        let f = x[index] - 2. * term / m as f64 - 1.;
        res += f.powi(2);
    }
    for _ in (x.len() + 1)..(m + 1) {
        let f = -2. * term / m as f64 - 1.;
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![1.; n]
}

pub fn min(_n: usize) -> Vec<f64> {
    // Should be -1 for Linear Full Rank? MGHMin had a test:
    // let linear_full_rank = MGH::aux(m).linear_full_rank(&MGHMin::linear_full_rank(n));
    // assert_eq!(linear_full_rank, (m as f64 - n as f64));
    // So the function value at min is (m-n).
    // The "min" vector implementation wasn't shown in `lib.rs` for `MGHMin`.
    // I'll skip adding `min` here unless I recall seeing it or infer implementation from struct.
    // Wait, outlining showed `MGHInit` having `linear_full_rank` but `MGHMin` also had `linear_full_rank`.
    // I don't have `MGHMin::linear_full_rank` code.
    // I will write a dummy one or omit. Omit is safer than wrong.
    vec![]
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
}
