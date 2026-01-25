pub fn extended_rosenbrock(x: &[f64]) -> f64 {
    if x.len() & 1 == 1 {
        panic!("number of auxiliary function must be in even");
    }
    let mut res = 0.0;

    for chunk in x.chunks_exact(2) {
        let &[x1, x2] = chunk else { unreachable!() };

        // f_{2i-1}(x) = 10(x_{2i} - x_{2i-1}^2)
        let term1 = 10.0 * (x2 - x1.powi(2));
        // f_{2i}(x) = 1 - x_{2i-1}
        let term2 = 1.0 - x1;

        res += term1.powi(2) + term2.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for (i, item) in vec.iter_mut().enumerate().take(n) {
        *item = if (i & 1) == 1 { 1.0 } else { -1.2 };
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
    fn test_extended_rosenbrock() {
        let n = 4;
        let x = init(n);
        let val = extended_rosenbrock(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let n = 4;
        let x = min(n);
        let val = extended_rosenbrock(&x);
        assert_eq!(val, 0.);
    }
}
