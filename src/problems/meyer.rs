use std::f64::consts::E;

pub fn meyer(x: &[f64]) -> f64 {
    let &[x1, x2, x3] = x else {
        panic!("input dimension must be 3");
    };

    let y = vec![
        34780., 28610., 23650., 19630., 16370., 13720., 11540., 9744., 8261., 7030., 6005., 5147.,
        4427., 3820., 3307., 2872.,
    ];

    let mut res = 0.0;

    for (i_idx, &yi) in y.iter().enumerate() {
        let i = i_idx as f64 + 1.;
        // t_i = 45 + 5i
        let t = 45. + 5. * i;
        
        // f_i(x) = x_1 exp[x_2 / (t_i + x_3)] - y_i
        let f = x1 * E.powf(x2 / (t + x3)) - yi;
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0.02, 4000., 250.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meyer() {
        let x = init();
        let val = meyer(&x);
        assert!(val.is_finite());
    }
}
