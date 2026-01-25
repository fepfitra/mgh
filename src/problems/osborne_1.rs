use std::f64::consts::E;

pub fn osborne_1(x: &[f64]) -> f64 {
    let &[x1, x2, x3, x4, x5] = x else {
        panic!("input dimension must be 5");
    };

    let y = [
        0.844, 0.908, 0.932, 0.936, 0.925, 0.908, 0.881, 0.850, 0.818, 0.784, 0.751, 0.718, 0.685,
        0.658, 0.628, 0.603, 0.580, 0.558, 0.538, 0.522, 0.506, 0.490, 0.478, 0.467, 0.457, 0.448,
        0.438, 0.431, 0.424, 0.420, 0.414, 0.411, 0.406,
    ];

    let mut res = 0.0;
    for (i_idx, &yi) in y.iter().enumerate() {
        let i = i_idx as f64 + 1.0;
        // t_i = 10(i - 1)
        let t = 10.0 * (i - 1.0);

        // f_i(x) = y_i - (x_1 + x_2 exp[-t_i x_4] + x_3 exp[-t_i x_5])
        let f = yi - (x1 + x2 * E.powf(-t * x4) + x3 * E.powf(-t * x5));
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0.5, 1.5, -1.0, 0.01, 0.02]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osborne_1() {
        let x = init();
        let val = osborne_1(&x);
        assert!(val.is_finite());
    }
}
