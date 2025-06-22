use std::vec;

struct MGHx0 {}
impl MGHx0 {
    fn rosenbrock() -> Vec<f64> {
        vec![1.0, 1.0]
    }
}

struct MGH {}

impl MGH {
    fn rosenbrock(x: Vec<f64>) -> f64 {
        if x.len() != 2 {
            panic!("Input vector must have exactly two elements.");
        }
        let x1 = x[0];
        let x2 = x[1];

        let f1 = 10.0 * (x2 - x1.powi(2));
        let f2 = 1.0 - x1;
        f1.powi(2) + f2.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rosenbrock() {
        let result = MGH::rosenbrock(MGHx0::rosenbrock());
        assert_eq!(result, 0.0);
    }
}
