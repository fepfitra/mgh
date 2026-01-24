use std::f64::consts::E;

pub fn box_3d(x: &[f64], m: usize) -> f64 {
    if x.len() != 3 {
        panic!("input dimension must be 3");
    }
    if m < 3 {
        panic!("number of auxiliary function must be at least 3");
    }
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];

    let mut res = 0.0;
    for i in 1..(m + 1) {
        let t = 0.1 * i as f64;
        let f = E.powf(-t * x1) - E.powf(-t * x2) - x3 * (E.powf(-t) - E.powf(-10. * t));
        res += f.powi(2)
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0., 10., 20.]
}

pub fn min() -> Vec<f64> {
    vec![1., 10., 1.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_3d() {
        let x = init();
        // Standard m usually depends on context, MGH often uses m=10 or similar for small n, but here it's variable.
        // We test with m=10
        let val = box_3d(&x, 10);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = box_3d(&x, 3); // min m is 3
        assert_eq!(val, 0.);
    }
}
