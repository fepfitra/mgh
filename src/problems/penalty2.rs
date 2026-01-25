use std::f64::consts::E;

pub fn penalty2(x: &[f64]) -> f64 {
    let a: f64 = 0.00001;
    // f_1(x) = x_1 - 0.2
    let mut res = (x[0] - 0.2).powi(2);

    for i in 2..(x.len() + 1) {
        let index = i - 1;

        // y_i = exp(i/10) + exp((i-1)/10)
        let y = E.powf(i as f64 / 10.) + E.powf((i as f64 - 1.) / 10.);

        // f_i(x) = sqrt(a)(exp(x_i/10) + exp(x_{i-1}/10) - y_i)
        let f = a.sqrt() * (E.powf(x[index] / 10.) + E.powf(x[index - 1] / 10.) - y);
        res += f.powi(2);
    }

    for i in (x.len() + 1)..2 * x.len() {
        let index = i - 1;
        // f_i(x) = sqrt(a)(exp(x_{i-n+1}/10) - exp(-1/10))
        let f = a.sqrt() * (E.powf(x[index - x.len() + 1] / 10.) - E.powf(-0.1));
        res += f.powi(2);
    }
    res += {
        let mut r = 0.;
        for j in 1..(x.len() + 1) {
            let jndex = j - 1;
            r += (x.len() as f64 - j as f64 + 1.) * x[jndex].powi(2);
        }
        // f_{2n}(x) = (sum (n-j+1)x_j^2) - 1
        (r - 1.).powi(2)
    };
    res
}

pub fn init(n: usize) -> Vec<f64> {
    vec![0.5; n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_penalty2() {
        let n = 3;
        let x = init(n);
        let val = penalty2(&x);
        assert!(val.is_finite());
    }
}
