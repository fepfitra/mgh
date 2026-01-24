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
    hellical_valley,
    freudenstein_and_roth,
    gaussian,
    powell_badly_scaled,
    brown_badly_scaled,
    powell_singular,
    beale,
    wood
);

impl_n!(
    broyden_tridiagonal,
    variably_dimensional,
    watson,
    penalty1,
    penalty2,
    trigonometric,
    extended_rosenbrock,
    broyden_banded,
    discrete_boundary_value,
    discrete_integral_equation,
    extended_powell_singular
);

impl_m!(
    jennrich_and_sampson,
    biggs_exp6,
    box_3d,
    brown_and_dennis,
    gulf_research_and_development
);

impl_nm!(
    chebyquad,
    linear_full_rank
);

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
    brown_badly_scaled,
    beale,
    gulf_research_and_development,
    box_3d,
    powell_singular,
    wood,
    biggs_exp6
);

impl_min_n!(
    variably_dimensional,
    extended_rosenbrock,
    extended_powell_singular
);
