pub fn bard(x: &[f64]) -> f64 {
    let &[x1, x2, x3] = x else {
        panic!("input dimension must be 3");
    };

    let y = [
        0.14, 0.18, 0.22, 0.25, 0.29, 0.32, 0.35, 0.39, 0.37, 0.58, 0.73, 0.96, 1.34, 2.10, 4.39,
    ];

    let mut res = 0.0;
    
    // The problem definition specifies m=15.
    for i_idx in 1..=15 {
        let u = i_idx as f64;
        let v = 16.0 - u;
        let w = if u < v { u } else { v };
        
        // f_i(x) = y_i - (x_1 + u_i / (v_i * x_2 + w_i * x_3))
        let denom = v * x2 + w * x3;
        let term = if denom.abs() < 1e-10 {
             // Handle potential division by zero
             if u > 0. { f64::INFINITY } else { 0. }
        } else {
             u / denom
        };
        
        let f = y[i_idx - 1] - (x1 + term);
        res += f.powi(2);
    }
    res
}

pub fn init() -> Vec<f64> {
    vec![1., 1., 1.]
}

pub fn min() -> Vec<f64> {
    // The value 8.21487... * 10^-3 at (0.0824..., 1.133..., 2.343...)
    vec![0.0824, 1.133, 2.343]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bard() {
        let x = init();
        let val = bard(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_min() {
        let x = min();
        let val = bard(&x);
        // f = 8.21487e-3
        assert!((val - 8.21487e-3).abs() < 1e-4);
    }
}
