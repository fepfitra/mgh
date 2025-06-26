# MGH: Moré, Garbow, and Hillstrom Test Functions

A pure Rust implementation of the classic **Moré, Garbow, and Hillstrom (MGH)** collection of test functions for unconstrained optimization algorithms.

This crate provides a standard suite of benchmark problems, essential for developers and researchers who need to test, validate, and compare the performance of optimization algorithms. The implementations are based on the original Fortran package and the definitions in the seminal paper:

> Moré, J. J., Garbow, B. S., & Hillstrom, K. E. (1981). Testing Unconstrained Optimization Software. *ACM Transactions on Mathematical Software (TOMS), 7(1), 17–41.* [https://dl.acm.org/doi/pdf/10.1145/355934.355936](https://dl.acm.org/doi/pdf/10.1145/355934.355936)

## Core Concepts

The library is organized into three main structs, each serving a distinct purpose:

1.  **`MGH`**: This is the primary struct used to evaluate the test functions themselves. Most functions are formulated as a sum of squares, $F(x) = \\sum\_{i=1}^{m} f\_i(x)^2$. The methods in `MGH` calculate this final scalar value $F(x)$.

2.  **`MGHInit`**: Provides the standard initial starting points ($x\_0$) for each test function, as specified in the MGH paper. Using these standard starting points is crucial for reproducing results and comparing different optimization algorithms fairly.

3.  **`MGHMin`**: Provides the known solution vectors ($x^\*$) for each function, which are the points where the function's value is at its global minimum. This is useful for verifying that an algorithm has found the correct solution.

## Installation

Add the crate to your `Cargo.toml` file:

```toml
[dependencies]
mgh = "0.1.0" # Replace with the latest version
```

## Usage and Examples

The general workflow is to:

1.  Get an initial starting vector from `MGHInit`.
2.  Use this vector in an optimization routine that repeatedly calls a function from `MGH`.
3.  Optionally, compare the result with the known minimum from `MGHMin`.

### Example 1: Evaluating a Simple Function (Gaussian)

This function has a fixed dimension ($n=3$) and does not require any auxiliary parameters.

```rust
use mgh::{MGH, MGHInit};

// 1. Get the standard initial vector for the Gaussian function.
let x0 = MGHInit::gaussian();
assert_eq!(x0, vec![0.4, 1.0, 0.0]);

// 2. Evaluate the function at this starting point.
let value = MGH::gaussian(x0);

println!("Gaussian function at standard start: {}", value);
// Output: Gaussian function at standard start: 0.00011279323789019634

// The known minimum for this function is very close to zero.
```

### Example 2: A Variable-Dimension Function (Extended Rosenbrock)

Many functions can be scaled to any dimension `n`.

```rust
use mgh::{MGH, MGHInit, MGHMin};

let n = 4; // Must be an even number for this function

// Get the initial vector for n=4
let x0 = MGHInit::extended_rosenbrock(n);

// Get the known solution vector for n=4
let x_min = MGHMin::extended_rosenbrock(n);
assert_eq!(x_min, vec![1.0, 1.0, 1.0, 1.0]);

// Evaluate the function at the known minimum
let min_value = MGH::extended_rosenbrock(x_min);

println!("Extended Rosenbrock (n=4) at its minimum: {}", min_value);
// Output: Extended Rosenbrock (n=4) at its minimum: 0
```

### Example 3: Functions Requiring an Auxiliary `m` Parameter

Some functions, like `Biggs EXP6`, are defined with a variable number of residual functions ($m \\ge n$). To use them, you first create an `MGH` instance with `MGH::aux(m)`.

```rust
use mgh::{MGH, MGHInit, MGHMin};

let n = 6;
let m = 13; // Number of residuals, m must be >= n

// Get the standard initial vector
let x0 = MGHInit::biggs_exp6();

// Evaluate the function by first creating an MGH instance with m
let value = MGH::aux(m).biggs_exp6(x0);

println!("Biggs EXP6 (m=13) at standard start: {}", value);

// Check the value at the known minimum
let x_min = MGHMin::biggs_exp6();
let min_value = MGH::aux(m).biggs_exp6(x_min);
assert!(min_value < 1e-9); // Should be very close to 0
```

## API Reference

The tables below list the available functions.

### `MGH` - Objective Functions

| Function Name | Dimension (`n`) | `m` Required? | Returns |
| :--- | :--- | :--- | :--- |
| `biggs_exp6` | 6 | Yes | `f64` |
| `gaussian` | 3 | No | `f64` |
| `powell_badly_scaled` | 2 | No | `f64` |
| `box_3d` | 3 | Yes | `f64` |
| `variably_dimensional`| Variable | No | `f64` |
| `watson` | 2 <= n <= 31 | No | `f64` |
| `penalty1` | Variable | No | `f64` |
| `penalty2` | Variable | No | `f64` |
| `brown_badly_scaled`| 2 | No | `f64` |
| `brown_and_dennis` | 4 | Yes | `f64` |
| `gulf_research_and_development` | 3 | Yes | `f64` |
| `trigonometric` | Variable | No | `f64` |
| `extended_rosenbrock`| Variable (even) | No | `f64` |
| `powell_singular` | 4 | No | `f64` |
| `beale` | 2 | No | `f64` |
| `wood` | 4 | No | `f64` |
| `extended_powell_singular` | Variable (mult of 4) | No | `f64` |
| `discrete_boundary_value` | Variable | No | `f64` |
| `discrete_integral_equation` | Variable | No | `f64` |
| `linear_full_rank` | Variable | Yes | `f64` |
| `chebyquad` | Variable | Yes | `Vec<f64>` |
| `broyden_banded` | Variable | No | `Vec<f64>` |

### `MGHInit` - Initial Vectors ($x\_0$)

| Function | Provides |
| :--- | :--- |
| `biggs_exp6()` | `Vec<f64>` for Biggs EXP6 |
| `gaussian()` | `Vec<f64>` for Gaussian |
| `powell_badly_scaled()` | `Vec<f64>` for Powell Badly Scaled |
| ... and so on for all other functions. |

### `MGHMin` - Solution Vectors ($x^\*$)

| Function | Provides |
| :--- | :--- |
| `brown_badly_scaled()` | Known minimizer for Brown Badly Scaled |
| `beale()` | Known minimizer for Beale |
| `gulf_research_and_development()` | Known minimizer for Gulf R\&D |
| `box_3d()` | Known minimizer for Box 3D |
| ... and so on for functions with known minimizers. |

## License

  * MIT license ([LICENSE-MIT](https://www.google.com/search?q=LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

## Contributing

Contributions are welcome\! Feel free to open an issue or submit a pull request.
