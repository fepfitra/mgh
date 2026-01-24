pub fn trigonometric(x: &[f64]) -> f64 {
    let mut res = 0.;
    for i in 1..(x.len() + 1) {
        let sum = {
            let mut s = 0.;
            for item in x {
                s += item.cos();
            }
            s
        };
        let f = x.len() as f64 - sum + i as f64 * (1. - x[i - 1].cos()) - x[i - 1].sin();
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![1. / (n as f64); n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric() {
        let n = 3;
        let x = init(n);
        let val = trigonometric(&x);
        assert!(val.is_finite());
    }
}
