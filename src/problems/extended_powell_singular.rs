pub fn extended_powell_singular(x: &[f64]) -> f64 {
    if !x.len().is_multiple_of(4) {
        panic!("input dimension must be multiple of 4");
    }
    let mut res = 0.;
    
    for chunk in x.chunks_exact(4) {
        let &[x1, x2, x3, x4] = chunk else { unreachable!() };

        // f_{4i-3}(x) = x_{4i-3} + 10x_{4i-2}
        let f1 = x1 + 10. * x2;
        // f_{4i-2}(x) = sqrt(5)(x_{4i-1} - x_{4i})
        let f2 = 5.0_f64.sqrt() * (x3 - x4);
        // f_{4i-1}(x) = (x_{4i-2} - 2x_{4i-1})^2
        let f3 = (x2 - 2. * x3).powi(2);
        // f_{4i}(x) = sqrt(10)(x_{4i-3} - x_{4i})^2
        let f4 = 10.0_f64.sqrt() * (x1 - x4).powi(2);

        res += f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for (i, item) in vec.iter_mut().enumerate() {
        *item = match i % 4 {
            0 => 3.0,
            1 => -1.0,
            2 => 0.0,
            3 => 1.0,
            _ => unreachable!(),
        };
    }
    vec
}

pub fn min(n: usize) -> Vec<f64> {
    vec![0.; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_powell_singular() {
        let n = 4;
        let x = init(n);
        let val = extended_powell_singular(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_value_n4() {
        let n = 4;
        let x = init(n);
        let val = extended_powell_singular(&x);
        // At x0 = (3, -1, 0, 1)
        // f1 = 3 - 10 = -7, f1^2 = 49
        // f2 = sqrt(5)(-1) = -sqrt(5), f2^2 = 5
        // f3 = (-1 - 0)^2 = 1, f3^2 = 1
        // f4 = sqrt(10)(2)^2 = 4sqrt(10), f4^2 = 160
        // Sum = 49 + 5 + 1 + 160 = 215
        assert!((val - 215.0).abs() < 1e-5);
    }

    #[test]
    fn test_min() {
        let n = 4;
        let x = min(n);
        let val = extended_powell_singular(&x);
        assert_eq!(val, 0.);
    }
}
