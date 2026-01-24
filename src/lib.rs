pub mod problems;

pub struct MGHInit {}
pub struct MGH {
    m: usize,
}

macro_rules! impl_static {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name() -> Vec<f64> {
                    problems::$name::init()
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(x: &[f64]) -> f64 {
                    problems::$name::$name(x)
                }
            )*
        }
    }
}

macro_rules! impl_n {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name(n: usize) -> Vec<f64> {
                    problems::$name::init(n)
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(x: &[f64]) -> f64 {
                    problems::$name::$name(x)
                }
            )*
        }
    }
}

macro_rules! impl_m {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name() -> Vec<f64> {
                    problems::$name::init()
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(&self, x: &[f64]) -> f64 {
                    problems::$name::$name(x, self.m)
                }
            )*
        }
    }
}

macro_rules! impl_nm {
    ($($name:ident),*) => {
        impl MGHInit {
            $(
                pub fn $name(n: usize) -> Vec<f64> {
                    problems::$name::init(n)
                }
            )*
        }
        impl MGH {
            $(
                pub fn $name(&self, x: &[f64]) -> f64 {
                    problems::$name::$name(x, self.m)
                }
            )*
        }
    }
}

impl_static!(
    beale,
    brown_badly_scaled,
    freudenstein_and_roth,
    gaussian,
    hellical_valley,
    powell_badly_scaled,
    powell_singular,
    rosenbrock,
    wood
);

impl_n!(
    broyden_banded,
    broyden_tridiagonal,
    discrete_boundary_value,
    discrete_integral_equation,
    extended_powell_singular,
    extended_rosenbrock,
    penalty1,
    penalty2,
    trigonometric,
    variably_dimensional,
    watson
);

impl_m!(
    biggs_exp6,
    box_3d,
    brown_and_dennis,
    gulf_research_and_development,
    jennrich_and_sampson
);

impl_nm!(chebyquad, linear_full_rank);

impl MGH {
    pub fn aux(m: usize) -> Self {
        Self { m }
    }
}

pub struct MGHMin {}

macro_rules! impl_min_static {
    ($($name:ident),*) => {
        impl MGHMin {
            $(
                pub fn $name() -> Vec<f64> {
                    problems::$name::min()
                }
            )*
        }
    }
}

macro_rules! impl_min_n {
    ($($name:ident),*) => {
        impl MGHMin {
            $(
                pub fn $name(n: usize) -> Vec<f64> {
                    problems::$name::min(n)
                }
            )*
        }
    }
}

impl_min_static!(
    beale,
    biggs_exp6,
    box_3d,
    brown_badly_scaled,
    gulf_research_and_development,
    powell_singular,
    rosenbrock,
    wood
);

impl_min_n!(
    extended_powell_singular,
    extended_rosenbrock,
    variably_dimensional
);
