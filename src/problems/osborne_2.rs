use std::f64::consts::E;

pub fn osborne_2(x: &[f64]) -> f64 {
    let &[x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11] = x else {
        panic!("input dimension must be 11");
    };

    let y = [
        1.366, 1.191, 1.112, 1.013, 0.991, 0.885, 0.831, 0.847, 0.786, 0.725, 0.746, 0.679, 0.608,
        0.655, 0.616, 0.606, 0.602, 0.626, 0.651, 0.724, 0.649, 0.649, 0.694, 0.644, 0.624, 0.661,
        0.612, 0.558, 0.533, 0.495, 0.500, 0.423, 0.395, 0.375, 0.372, 0.391, 0.396, 0.405, 0.428,
        0.429, 0.523, 0.562, 0.607, 0.653, 0.672, 0.708, 0.633, 0.668, 0.645, 0.632, 0.591, 0.559,
        0.597, 0.625, 0.739, 0.710, 0.729, 0.720, 0.636, 0.581, 0.428, 0.292, 0.162, 0.098, 0.054,
    ];

    let mut res = 0.0;
    for (i_idx, &yi) in y.iter().enumerate() {
        let i = i_idx as f64 + 1.0;
        // t_i = (i - 1) / 10
        let t = (i - 1.0) / 10.0;
        // println!("i: {}, t: {}, y: {}", i, t, yi);

        // f_i(x) = y_i - (x_1 exp[-t_i x_5] + x_2 exp[-(t_i - x_9)^2 x_6] + x_3 exp[-(t_i - x_10)^2 x_7] + x_4 exp[-(t_i - x_11)^2 x_8])
        let term1 = x1 * E.powf(-t * x5);
        let term2 = x2 * E.powf(-(t - x9).powi(2) * x6);
        let term3 = x3 * E.powf(-(t - x10).powi(2) * x7);
        let term4 = x4 * E.powf(-(t - x11).powi(2) * x8);

        let f = yi - (term1 + term2 + term3 + term4);
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![1.3, 0.65, 0.65, 0.7, 0.6, 3.0, 5.0, 7.0, 2.0, 4.5, 5.5]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_osborne_2() {
        let x = init();
        let val = osborne_2(&x);
        assert!(val.is_finite());
    }


}
