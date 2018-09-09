//! # Examples
//!
//! Create a graph for `(x + y) - 2*z`
//!
//! ```
//! use cagra::graph::*;
//!
//! let mut g: Graph<f64> = Graph::new();
//! // x = 1.0
//! let x = g.variable("x", 1.0).unwrap();
//! // y = 2.0
//! let y = g.variable("y", 2.0).unwrap();
//! // tmp = x + y
//! let tmp = g.add(x, y);
//! // z = -3.0
//! let z = g.variable("z", -3.0).unwrap();
//! // a = 2.0;
//! let a = g.variable("a", 2.0).unwrap();
//! // az = a * z
//! let az = g.mul(a, z);
//! // sum = tmp - az
//! let sum = g.sub(tmp, az);
//!
//! g.eval_value(sum, false);
//! let result = g.get_value(sum).unwrap();
//! assert!((result - 9.0).abs() < 1e-7);
//!
//! g.eval_deriv(sum);
//! let dx = g.get_deriv(x).unwrap();
//! let dy = g.get_deriv(y).unwrap();
//! let dz = g.get_deriv(z).unwrap();
//! assert!((dx - 1.0).abs() < 1e-7);
//! assert!((dy - 1.0).abs() < 1e-7);
//! assert!((dz + 2.0).abs() < 1e-7);
//! ```

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate num_complex;
extern crate num_traits;
extern crate petgraph;

pub mod error;
pub mod graph;
pub mod operator;
pub mod scalar;
