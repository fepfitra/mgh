use super::discrete_boundary_value;

pub fn discrete_integral_equation(x: &[f64]) -> f64 {
    let mut res = 0.;
    for i in 1..(x.len() + 1) {
        let index = i - 1;
        let h = 1. / (x.len() as f64 + 1.);
        let t = i as f64 * h;

        let term1 = {
            let mut sum = 0.;
            for j in 1..=i {
                let jndex = j - 1;
                let tj = j as f64 * h;
                sum += tj * (x[jndex] + tj + 1.).powi(3);
            }
            sum
        };

        let term2 = {
            let mut sum = 0.;
            for j in (i + 1)..=x.len() {
                let jndex = j - 1;
                let tj = j as f64 * h;
                sum += (1. - tj) * (x[jndex] + tj + 1.).powi(3);
            }
            sum
        };
        let f = x[index] + h * ((1. - t) * term1 + t * term2) / 2.;
        res += f.powi(2);
    }
    res
}

pub fn init(n: usize) -> Vec<f64> {
    discrete_boundary_value::init(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discrete_integral_equation() {
        let n = 3;
        let x = init(n);
        let val = discrete_integral_equation(&x);
        assert!(val.is_finite());
    }
}
