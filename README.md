# MGH: Moré, Garbow, and Hillstrom Test Functions

A pure Rust implementation of the classic **Moré, Garbow, and Hillstrom (MGH)** collection of test functions for unconstrained optimization algorithms.

This crate provides a standard suite of benchmark problems, essential for developers and researchers who need to test, validate, and compare the performance of optimization algorithms. The implementations are based on the original Fortran package and the definitions in the seminal paper:

> Moré, J. J., Garbow, B. S., & Hillstrom, K. E. (1981). Testing Unconstrained Optimization Software. *ACM Transactions on Mathematical Software (TOMS), 7(1), 17–41.* [https://dl.acm.org/doi/pdf/10.1145/355934.355936](https://dl.acm.org/doi/pdf/10.1145/355934.355936)

## Core Concepts

The library is organized into three main structs:

1.  **`MGH`**: Evaluates the objective functions. Most functions are formulated as a sum of squares, $F(x) = \sum_{i=1}^{m} f_i(x)^2$. All methods in `MGH` return a scalar `f64`.
2.  **`MGHInit`**: Provides the standard initial starting points ($x_0$) for each test function, as specified in the MGH paper.
3.  **`MGHMin`**: Provides known solution vectors ($x^*$) for functions where the global minimum is documented.

## Installation

Add the crate to your `Cargo.toml` file:

```toml
[dependencies]
mgh = "0.1.0"
```

## Usage and Examples

### Example 1: Fixed-Dimension Function (Gaussian)
Functions with fixed $n$ and $m$ use static methods.

```rust
use mgh::{MGH, MGHInit, MGHMin};

// 1. Get the standard starting point (n=3)
let x0 = MGHInit::gaussian();

// 2. Evaluate the objective function
let value = MGH::gaussian(&x0);
println!("Value at x0: {}", value);

// 3. (Optional) Verify at the known minimum
let x_min = MGHMin::gaussian();
let min_value = MGH::gaussian(&x_min);
assert!(min_value < 1e-10);
```

### Example 2: Variable-Dimension Function (Extended Rosenbrock)
Functions where $n$ can be specified. $m$ is determined by $n$.

```rust
use mgh::{MGH, MGHInit, MGHMin};

let n = 10; // Must be even for Extended Rosenbrock

let x0 = MGHInit::extended_rosenbrock(n);
let value = MGH::extended_rosenbrock(&x0);

let x_min = MGHMin::extended_rosenbrock(n);
let min_value = MGH::extended_rosenbrock(&x_min);
assert_eq!(min_value, 0.0);
```

### Example 3: Specifying Residuals (Biggs EXP6)
Functions requiring an explicit $m$ parameter use `MGH::aux(m)`.

```rust
use mgh::{MGH, MGHInit};

let m = 20; // Number of residuals
let x0 = MGHInit::biggs_exp6();

// Use .aux(m) to specify the number of residuals
let value = MGH::aux(m).biggs_exp6(&x0);
```

---

## API Reference

The tables below list all available test functions and their properties.

### 1. Fixed-Dimension Functions
These use simple static methods like `MGH::gaussian(&x)`.

| Function | $n$ | $m$ | Initial Vector | Solution Vector |
| :--- | :---: | :---: | :---: | :---: |
| `bard` | 3 | 15 | ✅ | ✅ |
| `beale` | 2 | 3 | ✅ | ✅ |
| `brown_badly_scaled` | 2 | 3 | ✅ | ✅ |
| `freudenstein_and_roth` | 2 | 2 | ✅ | ✅ |
| `gaussian` | 3 | 15 | ✅ | ✅ |
| `hellical_valley` | 3 | 3 | ✅ | ✅ |
| `kowalik_and_osborne` | 4 | 11 | ✅ | ❌ |
| `meyer` | 3 | 16 | ✅ | ❌ |
| `osborne_1` | 5 | 33 | ✅ | ❌ |
| `osborne_2` | 11 | 65 | ✅ | ❌ |
| `powell_badly_scaled` | 2 | 2 | ✅ | ✅ |
| `powell_singular` | 4 | 4 | ✅ | ✅ |
| `rosenbrock` | 2 | 2 | ✅ | ✅ |
| `wood` | 4 | 6 | ✅ | ✅ |

### 2. Variable-Dimension Functions
These use static methods but `MGHInit` and `MGHMin` require an `n` argument.

| Function | $m$ | Initial Vector | Solution Vector |
| :--- | :---: | :---: | :---: |
| `brown_almost_linear` | $n$ | ✅ | ✅ |
| `broyden_banded` | $n$ | ✅ | ❌ |
| `broyden_tridiagonal` | $n$ | ✅ | ❌ |
| `discrete_boundary_value` | $n$ | ✅ | ❌ |
| `discrete_integral_equation` | $n$ | ✅ | ❌ |
| `extended_powell_singular` | $n$ (mult of 4) | ✅ | ✅ |
| `extended_rosenbrock` | $n$ (even) | ✅ | ✅ |
| `penalty1` | $n + 1$ | ✅ | ❌ |
| `penalty2` | $2n$ | ✅ | ❌ |
| `trigonometric` | $n$ | ✅ | ❌ |
| `variably_dimensioned` | $n + 2$ | ✅ | ✅ |
| `watson` | 31 ($2 \le n \le 31$) | ✅ | ❌ |

### 3. Functions with Auxiliary $m$ Parameter
These require `MGH::aux(m)` to specify the number of residuals.

| Function | Dimension | Initial Vector | Solution Vector |
| :--- | :--- | :---: | :---: |
| `biggs_exp6` | $n = 6$ | ✅ | ✅ |
| `box_3d` | $n = 3$ | ✅ | ✅ |
| `brown_and_dennis` | $n = 4$ | ✅ | ✅ |
| `gulf_research_and_development` | $n = 3$ | ✅ | ✅ |
| `jennrich_and_sampson` | $n = 2$ | ✅ | ✅ |
| `chebyquad` | Variable $n$ | ✅ | ❌ |
| `linear_full_rank` | Variable $n$ | ✅ | ✅ |
| `linear_rank_1` | Variable $n$ | ✅ | ❌ |
| `linear_rank_1_zero` | Variable $n$ | ✅ | ❌ |

## License

* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
