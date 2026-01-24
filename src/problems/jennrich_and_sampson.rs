use std::f64::consts::E;

pub fn jennrich_and_sampson(x: &[f64], m: usize) -> f64 {
    if x.len() != 2 {
        panic!("input dimension must be 2");
    }
    if m < 2 { // Image says m=10 usually, but formula valid for m >= 2
         panic!("number of auxiliary function must be at least 2");
    }
    
    let x1 = x[0];
    let x2 = x[1];
    let mut res = 0.0;

    for i in 1..=m {
         let fi = 2.0 + 2.0 * i as f64 - (E.powf(i as f64 * x1) + E.powf(i as f64 * x2));
         res += fi.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0.3, 0.4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jennrich_and_sampson() {
        let x = init();
        let val = jennrich_and_sampson(&x, 10);
        assert!(val.is_finite());
    }
}
