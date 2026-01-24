pub fn beale(x: &[f64]) -> f64 {
    if x.len() != 2 {
        panic!("input dimension must be 2");
    }
    let x1 = x[0];
    let x2 = x[1];
    let y = [1.5, 2.25, 2.625];

    let mut res = 0.0;
    // f_i(x) = y_i - x_1(1 - x_2^i)
    // where y_1 = 1.5, y_2 = 2.25, y_3 = 2.625
    for i in 1..4 {
        let index = i - 1;
        let f = y[index] - x1 * (1. - x2.powi(i as i32));
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![1., 1.]
}

pub fn min() -> Vec<f64> {
    vec![3., 0.5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beale() {
        let x = init();
        let val = beale(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = beale(&x);
        assert_eq!(val, 0.);
    }
}
