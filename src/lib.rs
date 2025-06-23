use std::vec;
// use std::default::Default;

struct MGHInit {}

impl MGHInit {
    fn hellical_valley() -> Vec<f64> {
        vec![-1., 0., 0.]
    }

    fn biggs_EXP6() -> Vec<f64> {
        vec![1., 2., 1., 1., 1., 1.]
    }

    fn gaussian() -> Vec<f64> {
        vec![0.4, 1., 0.]
    }

    fn powell_badly_scaled() -> Vec<f64> {
        vec![0., 1.]
    }

    fn box_3d() -> Vec<f64> {
        vec![0., 10., 20.]
    }

    fn variably_dimensional(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = 1. - (i as f64 + 1.) / (n as f64);
        }
        vec
    }

    fn watson(n: usize) -> Vec<f64> {
        vec![0.; n]
    }

    fn penalty1(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = i as f64 + 1.;
        }
        vec
    }

    fn penalty2(n: usize) -> Vec<f64> {
        vec![0.5; n]
    }

    fn brown_badly_scaled() -> Vec<f64> {
        vec![1., 1.]
    }

    fn brown_and_dennis() -> Vec<f64> {
        vec![25., 5., -0.5, -1.]
    }

    fn gulf_research_and_development() -> Vec<f64> {
        vec![5., 2.5, 0.15]
    }

    fn trigonometric(n: usize) -> Vec<f64> {
        vec![1. / (n as f64); n]
    }

    fn extended_rosenbrock(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = if (i & 1) == 1 { 1.0 } else { -1.2 };
        }
        vec
    }

    fn powell_singular() -> Vec<f64> {
        vec![3., -1., 0., 1.]
    }

    fn beale() -> Vec<f64> {
        vec![1., 1.]
    }

    fn wood() -> Vec<f64> {
        vec![-3., -1., -3., -1.]
    }

    fn chebyquad(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = (i as f64 + 1.) / (n as f64 + 1.0)
        }
        vec
    }

    fn broyden_anded(n: usize) -> Vec<f64> {
        vec![-1.; n]
    }

    fn discrete_boundary_value(n: usize) -> Vec<f64> {
        let h = 1. / (n as f64 + 1.);
        let mut vec = vec![0.; n];
        for i in 0..n {
            let t = (i as f64 + 1.) * h;
            vec[i] = t * (t - 1.);
        }
        vec
    }

    fn discrete_integral_equation(n: usize) -> Vec<f64> {
        MGHInit::discrete_boundary_value(n)
    }

    fn linear_function_full_rank(n: usize) {
        vec![1.; n];
    }

    fn extended_powell_singular(n: usize) -> Vec<f64> {
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
}

struct MGH {}

impl MGH {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let hellical_valley = MGHInit::hellical_valley();
        assert_eq!(hellical_valley, vec![-1., 0., 0.]);

        let biggs_EXP6 = MGHInit::biggs_EXP6();
        assert_eq!(biggs_EXP6, vec![1., 2., 1., 1., 1., 1.]);

        let gaussian = MGHInit::gaussian();
        assert_eq!(gaussian, vec![0.4, 1., 0.]);

        let powell_badly_scaled = MGHInit::powell_badly_scaled();
        assert_eq!(powell_badly_scaled, vec![0., 1.]);

        let box_3d = MGHInit::box_3d();
        assert_eq!(box_3d, vec![0., 10., 20.]);

        let variably_dim = MGHInit::variably_dimensional(3);
        assert_eq!(variably_dim, vec![1. - 1. / 3., 1. - 2. / 3., 1. - 3. / 3.]);

        let watson = MGHInit::watson(3);
        assert_eq!(watson, vec![0., 0., 0.]);

        let penalty1 = MGHInit::penalty1(3);
        assert_eq!(penalty1, vec![1., 2., 3.]);

        let penalty2 = MGHInit::penalty2(3);
        assert_eq!(penalty2, vec![0.5, 0.5, 0.5]);

        let brown_badly_scaled = MGHInit::brown_badly_scaled();
        assert_eq!(brown_badly_scaled, vec![1., 1.]);

        let brown_and_dennis = MGHInit::brown_and_dennis();
        assert_eq!(brown_and_dennis, vec![25., 5., -0.5, -1.]);

        let gulf_research_and_development = MGHInit::gulf_research_and_development();
        assert_eq!(gulf_research_and_development, vec![5., 2.5, 0.15]);

        let trigonometric = MGHInit::trigonometric(3);
        assert_eq!(trigonometric, vec![1. / 3., 1. / 3., 1. / 3.]);

        let extended_rosenbrock = MGHInit::extended_rosenbrock(3);
        assert_eq!(extended_rosenbrock, vec![-1.2, 1., -1.2]);

        let powell_singular = MGHInit::powell_singular();
        assert_eq!(powell_singular, vec![3., -1., 0., 1.]);

        let discrete_boundary_value = MGHInit::discrete_boundary_value(3);
        assert_eq!(
            discrete_boundary_value,
            vec![
                (1. / 4.) * (-3. / 4.),
                (1. / 2.) * (-1. / 2.),
                (3. / 4.) * (-1. / 4.)
            ]
        );

        let discrete_integral_equation = MGHInit::discrete_integral_equation(3);
        assert_eq!(
            discrete_integral_equation,
            vec![
                (1. / 4.) * (-3. / 4.),
                (1. / 2.) * (-1. / 2.),
                (3. / 4.) * (-1. / 4.)
            ]
        );

        let extended_powell_singular = MGHInit::extended_powell_singular(4);
        assert_eq!(extended_powell_singular, vec![3., -1., 0., 1.])
    }
}
