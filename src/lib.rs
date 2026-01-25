pub mod problems;
#[macro_use]
mod macros;

pub struct MGHInit {}
pub struct MGH {
    m: usize,
}

impl_static!(
    bard,
    beale,
    brown_badly_scaled,
    freudenstein_and_roth,
    gaussian,
    hellical_valley,
    kowalik_and_osborne,
    meyer,
    osborne_1,
    osborne_2,
    powell_badly_scaled,
    powell_singular,
    rosenbrock,
    wood
);

impl_n!(
    broyden_banded,
    broyden_tridiagonal,
    brown_almost_linear,
    discrete_boundary_value,
    discrete_integral_equation,
    extended_powell_singular,
    extended_rosenbrock,
    penalty1,
    penalty2,
    trigonometric,
    variably_dimensioned,
    watson
);

impl_m!(
    biggs_exp6,
    box_3d,
    brown_and_dennis,
    gulf_research_and_development,
    jennrich_and_sampson
);

impl_nm!(chebyquad, linear_full_rank, linear_rank_1, linear_rank_1_zero);

impl MGH {
    pub fn aux(m: usize) -> Self {
        Self { m }
    }
}

pub struct MGHMin {}

impl_min_static!(
    bard,
    beale,
    biggs_exp6,
    box_3d,
    brown_badly_scaled,
    freudenstein_and_roth,
    gulf_research_and_development,
    hellical_valley,
    jennrich_and_sampson,
    powell_badly_scaled,
    powell_singular,
    rosenbrock,
    wood
);

impl_min_n!(
    extended_powell_singular,
    extended_rosenbrock,
    variably_dimensioned
);
