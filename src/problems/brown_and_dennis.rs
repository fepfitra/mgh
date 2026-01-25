use std::f64::consts::E;

pub fn brown_and_dennis(x: &[f64], m: usize) -> f64 {
    let &[x1, x2, x3, x4] = x else {
        panic!("input dimension must be 4");
    };

    if m < x.len() {
        panic!("number of auxiliary function must be at least n");
    }
    let mut res = 0.;

    for i in 1..(m + 1) {
        // let index = i - 1;
        let t = (i as f64) / 5.;
        let f = (x1 + t * x2 - E.powf(t)).powi(2) + (x3 + x4 * t.sin() - t.cos()).powi(2);
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![25., 5., -1., -1.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brown_and_dennis() {
        let x = init();
        let val = brown_and_dennis(&x, 10);
        assert!(val.is_finite());
    }
}
