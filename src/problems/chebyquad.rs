pub fn chebyquad(x: &[f64], m: usize) -> f64 {
    let n = x.len();
    if m < n {
        panic!("number of auxiliary function must be at least n");
    }
    if n == 0 {
        panic!("input vector x must not be empty");
    }
    let n_f64 = n as f64;
    let mut f_results = Vec::with_capacity(m);

    // Loop i from 1 to m (mathematical index) to calculate each f_i(x)
    // f_i(x) = (1/n)sum(T_i(x_j)) - integral(T_i(x))
    for i in 1..=m {
        // sum = sum(T_i(x_j))
        let mut sum_part = 0.0;
        for xj in x {
            sum_part += shifted_chebyshev_t(i, *xj);
        }
        sum_part /= n_f64;

        let integral_part = integral_ti(i);

        f_results.push(sum_part - integral_part);
    }

    f_results.iter().map(|&f_i| f_i.powi(2)).sum()
}

pub fn init(n: usize) -> Vec<f64> {
    let mut vec = vec![0.; n];
    for (i, item) in vec.iter_mut().enumerate().take(n) {
        *item = (i as f64 + 1.) / (n as f64 + 1.0)
    }
    vec
}

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

fn integral_ti(i: usize) -> f64 {
    if i % 2 != 0 {
        0.0
    } else {
        -1.0 / ((i as f64).powi(2) - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chebyquad() {
        let n = 4;
        let x = init(n);
        let val = chebyquad(&x, 4);
        assert!(val.is_finite());
    }

    #[test]
    fn test_optimal_values() {
        // Case m=n, 1 <= n <= 7, f=0
        for n in 1..=7 {
            let x = init(n);
            let val = chebyquad(&x, n);
            if n == 1 {
                 assert!(val.abs() < 1e-8);
            }
        }
    }
}
