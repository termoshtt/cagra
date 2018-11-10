//! Value and operators in calculation graph

use super::scalar::Field;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Unary {
    Neg,
}

impl Unary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Field>(&self, arg: A) -> A {
        match self {
            Unary::Neg => -arg,
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Field>(&self, _arg: A, deriv: A) -> A {
        match self {
            Unary::Neg => -deriv,
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
    pub fn eval_value<A: Field>(&self, lhs: A, rhs: A) -> A {
        match self {
            Binary::Add => lhs + rhs,
            Binary::Mul => lhs * rhs,
            Binary::Div => lhs / rhs,
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Field>(&self, lhs: A, rhs: A, deriv: A) -> (A, A) {
        match self {
            Binary::Add => (deriv, deriv),
            Binary::Mul => (rhs * deriv, lhs * deriv),
            Binary::Div => (deriv / rhs, -lhs * deriv / (rhs * rhs)),
        }
    }
}
