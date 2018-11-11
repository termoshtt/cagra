//! Value and operators in calculation graph

use cauchy::Scalar;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Unary {
    Neg,
    Pow2,
}

impl Unary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, arg: A) -> A {
        match self {
            Unary::Neg => -arg,
            Unary::Pow2 => arg * arg,
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, arg: A, deriv: A) -> A {
        match self {
            Unary::Neg => -deriv,
            Unary::Pow2 => A::from_f64(2.0).unwrap() * arg * deriv,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Binary {
    Add,
    Mul,
    Div,
}

impl Binary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, lhs: A, rhs: A) -> A {
        match self {
            Binary::Add => lhs + rhs,
            Binary::Mul => lhs * rhs,
            Binary::Div => lhs / rhs,
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, lhs: A, rhs: A, deriv: A) -> (A, A) {
        match self {
            Binary::Add => (deriv, deriv),
            Binary::Mul => (rhs * deriv, lhs * deriv),
            Binary::Div => (deriv / rhs, -lhs * deriv / (rhs * rhs)),
        }
    }
}
