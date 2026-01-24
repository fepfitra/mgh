use std::{
    f64::{
        self,
        consts::E,
    },
    vec,
};

/// Calculates the value of the i-th Chebyshev polynomial of the first kind,
/// shifted to the interval [0, 1].
/// Standard Chebyshev polynomials T_i(z) are defined on [-1, 1].
/// To shift them to x in [0, 1], we use the transformation z = 2x - 1.
///
/// They follow the recurrence relation:
/// T_0(z) = 1
/// T_1(z) = z
/// T_i(z) = 2z * T_{i-1}(z) - T_{i-2}(z)
fn shifted_chebyshev_t(i: usize, x: f64) -> f64 {
    // Map x from [0, 1] to z in [-1, 1]
    let z = 2.0 * x - 1.0;

    if i == 0 {
        return 1.0;
    }
    if i == 1 {
        return z;
    }

    // Use the recurrence relation for i >= 2
    let mut t_prev = 1.0; // T_0(z)
    let mut t_curr = z; // T_1(z)

    for _ in 2..=i {
        let t_next = 2.0 * z * t_curr - t_prev;
        t_prev = t_curr;
        t_curr = t_next;
    }

    t_curr
}

/// Calculates the definite integral of the i-th shifted Chebyshev polynomial
/// from 0 to 1, based on the pre-calculated formula in the image.
fn integral_ti(i: usize) -> f64 {
    // For i=1 (odd), we correctly return 0. The even case i^2-1 would be a division by zero.
    if i % 2 != 0 {
        // for i odd
        0.0
    } else {
        // for i even
        -1.0 / ((i as f64).powi(2) - 1.0)
    }
}

pub struct MGHInit {}

impl MGHInit {
    // pub fn hellical_valley() -> Vec<f64> {
    //     vec![-1., 0., 0.]
    // }

    pub fn broyden_tridiagonal(n: usize) -> Vec<f64> {
        vec![-1.; n]
    }

    pub fn biggs_exp6() -> Vec<f64> {
        vec![1., 2., 1., 1., 1., 1.]
    }

    pub fn gaussian() -> Vec<f64> {
        vec![0.4, 1., 0.]
    }

    pub fn powell_badly_scaled() -> Vec<f64> {
        vec![0., 1.]
    }

    pub fn box_3d() -> Vec<f64> {
        vec![0., 10., 20.]
    }

    pub fn variably_dimensional(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = 1. - (i as f64 + 1.) / (n as f64);
        }
        vec
    }

    pub fn watson(n: usize) -> Vec<f64> {
        vec![0.; n]
    }

    pub fn penalty1(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = i as f64 + 1.;
        }
        vec
    }

    pub fn penalty2(n: usize) -> Vec<f64> {
        vec![0.5; n]
    }

    pub fn brown_badly_scaled() -> Vec<f64> {
        vec![1., 1.]
    }

    pub fn brown_and_dennis() -> Vec<f64> {
        vec![25., 5., -1., -1.]
    }

    pub fn gulf_research_and_development() -> Vec<f64> {
        vec![5., 2.5, 0.15]
    }

    pub fn trigonometric(n: usize) -> Vec<f64> {
        vec![1. / (n as f64); n]
    }

    pub fn extended_rosenbrock(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = if (i & 1) == 1 { 1.0 } else { -1.2 };
        }
        vec
    }

    pub fn powell_singular() -> Vec<f64> {
        vec![3., -1., 0., 1.]
    }

    pub fn beale() -> Vec<f64> {
        vec![1., 1.]
    }

    pub fn wood() -> Vec<f64> {
        vec![-3., -1., -3., -1.]
    }

    pub fn chebyquad(n: usize) -> Vec<f64> {
        let mut vec = vec![0.; n];
        for i in 0..n {
            vec[i] = (i as f64 + 1.) / (n as f64 + 1.0)
        }
        vec
    }

    pub fn broyden_banded(n: usize) -> Vec<f64> {
        vec![-1.; n]
    }

    pub fn discrete_boundary_value(n: usize) -> Vec<f64> {
        let h = 1. / (n as f64 + 1.);
        let mut vec = vec![0.; n];
        for i in 0..n {
            let t = (i as f64 + 1.) * h;
            vec[i] = t * (t - 1.);
        }
        vec
    }

    pub fn discrete_integral_equation(n: usize) -> Vec<f64> {
        MGHInit::discrete_boundary_value(n)
    }

    pub fn linear_full_rank(n: usize) -> Vec<f64> {
        vec![1.; n]
    }

    pub fn extended_powell_singular(n: usize) -> Vec<f64> {
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

pub struct MGH {
    m: usize,
}

impl MGH {
    pub fn aux(m: usize) -> Self {
        Self { m }
    }

    // pub fn hellical_valley(x: Vec<f64>) -> f64 {
    //     if x.len() != 3 {
    //         panic!("input dimension must be 3");
    //     }
    //
    //     let x1 = x[0];
    //     let x2 = x[1];
    //     let x3 = x[2];
    //
    //     let theta = if x1 > 0. {
    //         (x2 / x1).atan() / (2. * PI)
    //     } else {
    //         (x2 / x1).atan() / (2. * PI) + 0.5
    //     };
    //
    //     let f1 = 10. * (x3 - 10. * theta);
    //     let f2 = 10. * ((x1.powi(2) + x2.powi(2)).sqrt() - 1.);
    //     let f3 = x3;
    //
    //     f1.powi(2) + f2.powi(2) + f3.powi(2)
    // }

    pub fn broyden_tridiagonal(x: &[f64]) -> f64 {
        let mut new_x = vec![0.];
        new_x.extend_from_slice(x);
        new_x.push(0.);

        let mut res = 0.;
        for i in 1..(new_x.len() - 1) {
            let f = (3. - 2. * new_x[i]) * new_x[i] - new_x[i - 1] - 2. * new_x[i + 1] + 1.;
            res += f.powi(2);
        }
        res
    }

    pub fn biggs_exp6(self: &Self, x: &[f64]) -> f64 {
        if self.m < 6 {
            panic!("number of auxiliary function must be at least 6");
        }
        if x.len() != 6 {
            panic!("input dimension must be 6");
        }
        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];
        let x4 = x[3];
        let x5 = x[4];
        let x6 = x[5];

        let mut res = 0.0;
        for i in 1..(self.m + 1) {
            let t = 0.1 * i as f64;
            let y = E.powf(-t) - 5. * E.powf(-10. * t) + 3. * E.powf(-4. * t);
            let f = x3 * E.powf(-t * x1) - x4 * E.powf(-t * x2) + x6 * E.powf(-t * x5) - y;
            res += f.powi(2);
        }
        res
    }

    pub fn gaussian(x: &[f64]) -> f64 {
        if x.len() != 3 {
            panic!("input dimension must be 3");
        }
        let y = vec![
            0.0009, 0.0044, 0.0175, 0.0540, 0.1295, 0.2420, 0.3521, 0.3989, 0.3521, 0.2420, 0.1295,
            0.0540, 0.0175, 0.0044, 0.0009,
        ];
        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];

        let mut res = 0.0;

        for index in 0..15 {
            let i = index as f64 + 1.;
            let t = (8. - i) / 2.;
            let f = x1 * E.powf((-x2 * (t - x3).powi(2)) / 2.) - y[index];
            res += f.powi(2);
        }
        res
    }
    pub fn powell_badly_scaled(x: &[f64]) -> f64 {
        if x.len() != 2 {
            panic!("input dimension must be 2");
        }
        let x1 = x[0];
        let x2 = x[1];

        let f1 = 10000. * x1 * x2 - 1.;
        let f2 = E.powf(-x1) + E.powf(-x2) - 1.0001;
        f1.powi(2) + f2.powi(2)
    }

    pub fn box_3d(self: &Self, x: &[f64]) -> f64 {
        if x.len() != 3 {
            panic!("input dimension must be 3");
        }
        if self.m < 3 {
            panic!("number of auxiliary function must be at least 3");
        }
        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];

        let mut res = 0.0;
        for i in 1..(self.m + 1) {
            let t = 0.1 * i as f64;
            let f = E.powf(-t * x1) - E.powf(-t * x2) - x3 * (E.powf(-t) - E.powf(-10. * t));
            res += f.powi(2)
        }
        res
    }

    pub fn variably_dimensional(x: &[f64]) -> f64 {
        let mut res = 0.0;
        for i in 0..x.len() {
            res += (x[i] - 1.).powi(2);
        }
        let fnplus1 = {
            let mut f = 0.0;
            for i in 1..(x.len() + 1) {
                let index = i - 1;
                f += i as f64 * (x[index] - 1.);
            }
            f
        };
        res + fnplus1.powi(2) + fnplus1.powi(4)
    }

    pub fn watson(x: &[f64]) -> f64 {
        if x.len() < 2 || x.len() > 31 {
            panic!("input dimension must be 2<=n<=31");
        }
        let mut res = 0.0;
        for i in 1..30 {
            let t = i as f64 / 29.;
            let l1 = {
                let mut r = 0.0;
                for j in 2..(x.len() + 1) {
                    let jndex = j - 1;
                    r += (j as f64 - 1.) * x[jndex] * t.powi(j as i32 - 2);
                }
                r
            };
            let l2 = {
                let mut r = 0.0;
                for j in 1..(x.len() + 1) {
                    let jndex = j - 1;
                    r += (x[jndex] * t.powi(j as i32 - 1)).powi(2)
                }
                r.powi(2)
            };
            res += (l1 - l2 - 1.).powi(2);
        }
        let x1 = x[0];
        let x2 = x[1];
        res += x1.powi(2);
        res += (x2 - x1.powi(2) - 1.).powi(2);
        res
    }

    pub fn penalty1(x: &[f64]) -> f64 {
        let a: f64 = 0.00001;
        let mut res = 0.0;
        for index in 0..x.len() {
            let f = a.sqrt() * (x[index] - 1.);
            res += f.powi(2);
        }
        res += {
            let mut r = 0.;
            for i in 0..x.len() {
                r += x[i].powi(2);
            }
            (r - 0.25).powi(2)
        };
        res
    }

    pub fn penalty2(x: &[f64]) -> f64 {
        let a: f64 = 0.00001;
        let mut res = x[0] - 0.2;

        for i in 2..(x.len() + 1) {
            let index = i - 1;
            let y = E.powf(i as f64 / 10.) + E.powf((i as f64 - 1.) / 10.);
            let f = a.sqrt() * (E.powf(x[index] / 10.) + E.powf(x[index - 1] / 10.) - y);
            res += f.powi(2);
        }

        for i in (x.len() + 1)..2 * x.len() {
            let index = i - 1;
            let f = a.sqrt() * (E.powf(x[index - x.len() + 1] / 10.) - E.powf(-0.1));
            res += f.powi(2);
        }
        res += {
            let mut r = 0.;
            for j in 1..(x.len() + 1) {
                let jndex = j - 1;
                r += (x.len() as f64 - j as f64 + 1.) * x[jndex].powi(2);
            }
            (r - 1.).powi(2)
        };
        res
    }

    pub fn brown_badly_scaled(x: &[f64]) -> f64 {
        let x1 = x[0];
        let x2 = x[1];

        let f1 = x1 - 1000000.;
        let f2 = x2 - 0.000002;
        let f3 = x1 * x2 - 2.;

        f1.powi(2) + f2.powi(2) + f3.powi(2)
    }

    pub fn brown_and_dennis(self: &Self, x: &[f64]) -> f64 {
        if x.len() != 4 {
            panic!("input dimension must be 4");
        }

        if self.m < x.len() {
            panic!("number of auxiliary function must be at least n");
        }

        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];
        let x4 = x[3];
        let mut res = 0.;

        for i in 1..(self.m + 1) {
            // let index = i - 1;
            let t = (i as f64) / 5.;
            let f = (x1 + t * x2 - E.powf(t)).powi(2) + (x3 + x4 * t.sin() - t.cos()).powi(2);
            res += f.powi(2);
        }
        res
    }

    pub fn gulf_research_and_development(self: &Self, x: &[f64]) -> f64 {
        if self.m < 3 || self.m > 100 {
            panic!("number of auxiliary function must be in 3 <= m <= 100");
        }
        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];
        let mut res = 0.;
        for i in 1..(self.m + 1) {
            let t = i as f64 / 100.;
            let y = 25. + (-50. * t.ln()).powf(2. / 3.);

            let f = E.powf(-(y * self.m as f64 * i as f64 * x2).abs().powf(x3) / x1) - t;
            res += f.powi(2);
        }
        res
    }
    pub fn trigonometric(x: &[f64]) -> f64 {
        let mut res = 0.;
        for i in 1..(x.len() + 1) {
            let sum = {
                let mut s = 0.;
                for j in 0..x.len() {
                    s += x[j].cos();
                }
                s
            };
            let f = x.len() as f64 - sum + i as f64 * (1. - x[i - 1].cos()) - x[i - 1].sin();
            res += f.powi(2);
        }
        res
    }

    pub fn extended_rosenbrock(x: &[f64]) -> f64 {
        if x.len() & 1 == 1 {
            panic!("number of auxiliary function must be in even");
        }
        let mut res = 0.0;
        let num_pairs = x.len() / 2;

        for i in 0..num_pairs {
            let x_odd_idx = 2 * i;
            let x_even_idx = 2 * i + 1;

            let x1 = x[x_odd_idx];
            let x2 = x[x_even_idx];

            let term1 = 10.0 * (x2 - x1.powi(2));
            let term2 = 1.0 - x1;

            res += term1.powi(2) + term2.powi(2);
        }
        res
    }

    pub fn powell_singular(x: &[f64]) -> f64 {
        if x.len() != 4 {
            panic!("input dimension must be 4");
        }

        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];
        let x4 = x[3];

        let f1 = x1 + 10. * x2;
        let f2 = 5.0_f64.sqrt() * (x3 - x4);
        let f3 = (x2 - 2. * x3).powi(2);
        let f4 = 10.0_f64.sqrt() * (x1 - x4).powi(2);

        f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2)
    }

    pub fn beale(x: &[f64]) -> f64 {
        if x.len() != 2 {
            panic!("input dimension must be 2");
        }
        let x1 = x[0];
        let x2 = x[1];
        let y = vec![1.5, 2.25, 2.625];

        let mut res = 0.0;
        for i in 1..4 {
            let index = i - 1;
            let f = y[index] - x1 * (1. - x2.powi(i as i32));
            res += f.powi(2);
        }
        res
    }

    pub fn wood(x: &[f64]) -> f64 {
        if x.len() != 4 {
            panic!("input dimension must be 4");
        }
        let x1 = x[0];
        let x2 = x[1];
        let x3 = x[2];
        let x4 = x[3];
        let f1 = 10. * (x2 - x1.powi(2));
        let f2 = 1. - x1;
        let f3 = 90.0_f64.sqrt() * (x4 - x3.powi(2));
        let f4 = 1. - x3;
        let f5 = 10.0_f64.sqrt() * (x2 + x4 - 2.);
        let f6 = 10.0_f64.sqrt().recip() * (x2 - x4);

        f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2) + f5.powi(2) + f6.powi(2)
    }

    /// Implements the Chebyquad function.
    ///
    /// This function calculates a vector f(x) of size m, where each component f_i is
    /// defined by the formula:
    /// f_i(x) = (1/n) * Σ_{j=1 to n} T_i(x_j) - ∫_0^1 T_i(x) dx
    ///
    /// # Arguments
    ///
    /// * `x`: A slice representing the input vector x = (x_1, ..., x_n).
    /// * `m`: The number of functions f_i to compute.
    ///
    /// # Returns
    ///
    /// A `Vec<f64>` containing the result vector f(x).
    ///
    pub fn chebyquad(self: &Self, x: &[f64]) -> f64 {
        let n = x.len();
        if self.m < n {
            panic!("number of auxiliary function must be at least n");
        }
        if n == 0 {
            panic!("input vector x must not be empty");
        }
        let n_f64 = n as f64;
        let mut f_results = Vec::with_capacity(self.m);

        // Loop i from 1 to m (mathematical index) to calculate each f_i(x)
        for i in 1..=self.m {
            // --- Calculate Summation Part ---
            // (1/n) * Σ_{j=1 to n} T_i(x_j)
            let mut sum_part = 0.0;
            for xj in x {
                sum_part += shifted_chebyshev_t(i, *xj);
            }
            sum_part /= n_f64;

            // --- Calculate Integral Part ---
            let integral_part = integral_ti(i);

            // --- Combine and store the result for f_i ---
            f_results.push(sum_part - integral_part);
        }

        f_results.iter().map(|&f_i| f_i.powi(2)).sum()
    }

    pub fn broyden_banded(x: &[f64]) -> f64 {
        let n = x.len();
        let ml = 5;
        let mu = 1;

        // store the results f_i(x) for i = 1 to n.
        let mut f = Vec::with_capacity(n);

        // Loop to calculate each f_i(x).
        // Note: Rust uses 0-based indexing (0 to n-1), while the formula
        // uses 1-based indexing (1 to n). We will use `i` as the 0-based index.
        for i in 0..n {
            let xi = x[i];

            // first part of the formula: x_i(2 + 5x_i^2) + 1
            let first_part = xi * (2.0 + 5.0 * xi.powi(2)) + 1.0;

            // Calculate the summation part: Σ_{j ∈ J_i} x_j(1 + x_j)
            let mut sum = 0.0;

            // Determine the bounds for the inner loop based on J_i.
            // We convert the 1-based math formula to 0-based Rust indices.
            // Math: max(1, i - m_l)  -> Rust: (i + 1).saturating_sub(m_l).saturating_sub(1)
            // A simpler way is to work with isize or use saturating_sub.
            let lower_j = i.saturating_sub(ml);

            // Math: min(n, i + m_u)  -> Rust: (i + 1 + m_u).min(n) - 1
            let upper_j = (i + mu).min(n - 1);

            // Loop over the "band" of j indices.
            for j in lower_j..=upper_j {
                // The condition `j ≠ i` from the definition of J_i.
                if i == j {
                    continue;
                }

                let xj = x[j];
                sum += xj * (1.0 + xj);
            }

            // Combine the parts to get the final f_i(x)
            f.push(first_part - sum);
        }

        f.iter().map(|&fi| fi.powi(2)).sum()
    }

    pub fn discrete_boundary_value(x: &[f64]) -> f64 {
        let mut res = 0.;
        for i in 1..(x.len() + 1) {
            let index = i - 1;
            let h = 1. / (x.len() as f64 + 1.);
            let t = i as f64 * h;

            let plus1 = if index + 1 < x.len() {
                x[index + 1]
            } else {
                0.
            };
            let minus1 = if index > 0 { x[index - 1] } else { 0. };

            let f = 2. * x[index] - minus1 - plus1 + h.powi(2) * (x[index] + t + 1.).powi(3) / 2.;

            res += f.powi(2);
        }
        res
    }
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

    pub fn extended_powell_singular(x: &[f64]) -> f64 {
        if x.len() & 3 != 0 {
            panic!("input dimension must be in multiple of 4");
        }
        let mut res = 0.;
        for i in 0..(x.len() / 4) {
            let x_1 = 4 * i;
            let x_2 = 4 * i + 1;
            let x_3 = 4 * i + 2;
            let x_4 = 4 * i + 3;

            let f1 = x[x_1] + 10. * x[x_2];
            let f2 = 5.0_f64.sqrt() * (x[x_3] - x[x_4]);
            let f3 = (x[x_2] - 2. * x[x_3]).powi(2);
            let f4 = 10.0_f64.sqrt() * (x[x_1] - x[x_4]).powi(2);
            res += f1.powi(2) + f2.powi(2) + f3.powi(2) + f4.powi(2);
        }
        res
    }

    pub fn linear_full_rank(self: &Self, x: &[f64]) -> f64 {
        if self.m < x.len() {
            panic!("number of auxiliary function must be at least n");
        }

        let mut res = 0.;

        let term = {
            let mut sum = 0.;
            for j in 1..(x.len() + 1) {
                let jndex = j - 1;
                sum += x[jndex];
            }
            sum
        };

        for i in 1..(x.len() + 1) {
            let index = i - 1;
            let f = x[index] - 2. * term / self.m as f64 - 1.;
            res += f.powi(2);
        }
        for _ in (x.len() + 1)..(self.m + 1) {
            let f = -2. * term / self.m as f64 - 1.;
            res += f.powi(2);
        }
        res
    }
}

pub struct MGHMin {}
impl MGHMin {
    pub fn brown_badly_scaled() -> Vec<f64> {
        vec![1000000., 0.000002]
    }
    pub fn beale() -> Vec<f64> {
        vec![3., 0.5]
    }
    pub fn gulf_research_and_development() -> Vec<f64> {
        vec![50., 25.0, 1.5]
    }
    pub fn box_3d() -> Vec<f64> {
        vec![1., 10., 1.]
    }
    pub fn powell_singular() -> Vec<f64> {
        vec![0., 0., 0., 0.]
    }
    pub fn wood() -> Vec<f64> {
        vec![1., 1., 1., 1.]
    }
    pub fn biggs_exp6() -> Vec<f64> {
        vec![1., 10., 1., 5., 4., 3.]
    }
    pub fn extended_rosenbrock(n: usize) -> Vec<f64> {
        vec![1.; n]
    }
    pub fn extended_powell_singular(n: usize) -> Vec<f64> {
        if n & 3 != 0 {
            panic!("input dimension must be in multiple of 4");
        }
        vec![0.; n]
    }
    pub fn variably_dimensional(n: usize) -> Vec<f64> {
        vec![1.; n]
    }
    pub fn linear_full_rank(n: usize) -> Vec<f64> {
        vec![-1.; n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        // let hellical_valley = MGHInit::hellical_valley();
        // assert_eq!(hellical_valley, vec![-1., 0., 0.]);

        let biggs_exp6 = MGHInit::biggs_exp6();
        assert_eq!(biggs_exp6, vec![1., 2., 1., 1., 1., 1.]);

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
        assert_eq!(brown_and_dennis, vec![25., 5., -1., -1.]);

        let gulf_research_and_development = MGHInit::gulf_research_and_development();
        assert_eq!(gulf_research_and_development, vec![5., 2.5, 0.15]);

        let trigonometric = MGHInit::trigonometric(3);
        assert_eq!(trigonometric, vec![1. / 3., 1. / 3., 1. / 3.]);

        let extended_rosenbrock = MGHInit::extended_rosenbrock(3);
        assert_eq!(extended_rosenbrock, vec![-1.2, 1., -1.2]);

        let powell_singular = MGHInit::powell_singular();
        assert_eq!(powell_singular, vec![3., -1., 0., 1.]);

        let broyden_banded = MGHInit::broyden_banded(3);
        assert_eq!(broyden_banded, vec![-1., -1., -1.]);

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

    #[test]
    fn test_min() {
        // let hellical_valley = MGH::hellical_valley(vec![-1., 0., 0.]);
        // assert_eq!(hellical_valley, 0.0);

        // let powell_badly_scaled = MGH::powell_badly_scaled(vec![0., 9.106]);
        // assert!((powell_badly_scaled - 0.0001).abs() < 1.);

        let brown_badly_scaled = MGH::brown_badly_scaled(&MGHMin::brown_badly_scaled());
        assert_eq!(brown_badly_scaled, 0.);

        let beale = MGH::beale(&MGHMin::beale());
        assert_eq!(beale, 0.);

        let gulf_research_and_development =
            MGH::aux(3).gulf_research_and_development(&MGHMin::gulf_research_and_development());
        assert!((gulf_research_and_development - 0.).abs() < 1e-2);

        let box_3d = MGH::aux(3).box_3d(&MGHMin::box_3d());
        assert_eq!(box_3d, 0.);

        let powell_singular = MGH::powell_singular(&MGHMin::powell_singular());
        assert_eq!(powell_singular, 0.);

        let wood = MGH::wood(&MGHMin::wood());
        assert_eq!(wood, 0.);

        let biggs_exp = MGH::aux(6).biggs_exp6(&MGHMin::biggs_exp6());
        assert_eq!(biggs_exp, 0.);

        let extended_rosenbrock = MGH::extended_rosenbrock(&MGHMin::extended_rosenbrock(4));
        assert_eq!(extended_rosenbrock, 0.);

        let extended_powell_singular =
            MGH::extended_powell_singular(&MGHMin::extended_powell_singular(4));
        assert_eq!(extended_powell_singular, 0.);

        let variably_dim = MGH::variably_dimensional(&MGHMin::variably_dimensional(3));
        assert_eq!(variably_dim, 0.);

        let m = 6;
        let n = 6;
        let linear_full_rank = MGH::aux(m).linear_full_rank(&MGHMin::linear_full_rank(n));
        assert_eq!(linear_full_rank, (m as f64 - n as f64));
    }
    #[test]
    fn test_broyden_tridiagonal() {
        let n = 10;
        let x = MGHInit::broyden_tridiagonal(n);
        assert_eq!(x.len(), n);
        assert_eq!(x[0], -1.0);
        
        // should not panic
        let val = MGH::broyden_tridiagonal(&x);
        assert!(val.is_finite());
    }

    #[test]
    fn test_chebyquad() {
        let n = 4;
        let m = 4;
        let x = MGHInit::chebyquad(n);
        let mgh = MGH::aux(m);
        let val = mgh.chebyquad(&x);
        assert!(val.is_finite());
    }
}
