//! cagra
//! =====
//!
//! Tiny CAlculation GRAph library
//!
//! Features
//! --------
//!
//! - No frontend and No backend.
//! - Adjacency list graph based on [petgraph](https://github.com/bluss/petgraph)
//! - Serialization with [serde](https://github.com/serde-rs/serde)
//!
//! Examples
//! --------
//!
//! Create a graph for `z = (x + y) - 2*x*y`
//!
//! ```
//! # use approx::abs_diff_eq;
//! use cagra::graph::*;
//!
//! let mut g: Graph<f64> = Graph::new();
//! let x = g.variable("x", 1.0).unwrap();
//! let y = g.variable("y", 3.0).unwrap();
//! let x_y = g.add(x, y);
//! let xy  = g.mul(x, y);
//! let a   = g.constant(2.0);
//! let axy = g.mul(a, xy);
//! let sum = g.sub(x_y, axy);
//!
//! let result = g.eval_value(sum).unwrap();
//! abs_diff_eq!(result, -2.0);
//!
//! g.eval_deriv(sum);
//! let dx = g.get_deriv(x).unwrap();
//! let dy = g.get_deriv(y).unwrap();
//! abs_diff_eq!(dx, -5.0);
//! abs_diff_eq!(dy, -1.0);
//! ```

#[doc(hidden)]
pub extern crate cagra_parser as parser;

#[macro_use]
pub mod graph;
pub mod error;
pub mod operator;
