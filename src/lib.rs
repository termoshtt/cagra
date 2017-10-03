//! # Examples
//!
//! Create a graph for `(x + y) - z`
//!
//! ```
//! use cagra::graph::*;
//!
//! let mut g: Graph<f64> = Graph::new();
//! // x = 1.0
//! let x = g.variable("x", 1.0);
//! // y = 2.0
//! let y = g.variable("y", 2.0);
//! // tmp = x + y
//! let tmp = g.add(x, y);
//! // z = -3.0
//! let z = g.variable("z", -3.0);
//! // sum = tmp - z
//! let sum = g.sub(tmp, z);
//!
//! g.eval_value(sum, false);
//! let result = g.get_value(sum).unwrap();
//! assert!((result - 6.0).abs() < 1e-7);
//!
//! g.eval_deriv(sum);
//! let dx = g.get_deriv(x).unwrap();
//! let dy = g.get_deriv(y).unwrap();
//! let dz = g.get_deriv(z).unwrap();
//! assert!((dx - 1.0).abs() < 1e-7);
//! assert!((dy - 1.0).abs() < 1e-7);
//! assert!((dz + 1.0).abs() < 1e-7);
//! ```

#[macro_use]
extern crate procedurals;
extern crate petgraph;
extern crate ndarray_linalg;

pub mod graph;
pub mod operators;
pub mod error;
