pub fn extended_powell_singular(x: &[f64]) -> f64 {
    if x.len() & 3 != 0 {
        panic!("input dimension must be in multiple of 4");
    }
    let mut res = 0.;
    for i in 0..(x.len() / 4) {
        let x_1 = 4 * i;
        let x_2 = 4 * i + 1;
        let x_3 = 4 * i + 2;
        let x_4 = 4 * i + 3;

        let f1 = x[x_1] + 10. * x[x_2];
        let f2 = 5.0_f64.sqrt() * (x[x_3] - x[x_4]);
        let f3 = (x[x_2] - 2. * x[x_3]).powi(2);
        let f4 = 10.0_f64.sqrt() * (x[x_1] - x[x_4]).powi(2);
        res += f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for i in 0..n {
        vec[i] = if (i & 3) == 3 {
            1.
        } else if (i & 2) == 2 {
            0.
        } else if (i & 1) == 1 {
            -1.
        } else {
            3.
        }
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
    fn test_min() {
        let n = 4;
        let x = min(n);
        let val = extended_powell_singular(&x);
        assert_eq!(val, 0.);
    }
}
