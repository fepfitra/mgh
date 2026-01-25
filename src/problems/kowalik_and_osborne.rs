pub fn kowalik_and_osborne(x: &[f64]) -> f64 {
    let &[x1, x2, x3, x4] = x else {
        panic!("input dimension must be 4");
    };

    let y = [
        0.1957, 0.1947, 0.1735, 0.1600, 0.0844, 0.0627, 0.0456, 0.0342, 0.0323, 0.0235, 0.0246,
    ];
    let u = [
        4.0000, 2.0000, 1.0000, 0.5000, 0.2500, 0.1670, 0.1250, 0.1000, 0.0833, 0.0714, 0.0625,
    ];

    let mut res = 0.0;
    for i in 0..11 {
        let yi: f64 = y[i];
        let ui: f64 = u[i];

        // f_i(x) = y_i - (x_1(u_i^2 + u_i x_2)) / (u_i^2 + u_i x_3 + x_4)
        let num = x1 * (ui.powi(2) + ui * x2);
        let den = ui.powi(2) + ui * x3 + x4;
        let f = yi - num / den;
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![0.25, 0.39, 0.415, 0.39]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kowalik_and_osborne() {
        let x = init();
        let val = kowalik_and_osborne(&x);
        assert!(val.is_finite());
    }
}
