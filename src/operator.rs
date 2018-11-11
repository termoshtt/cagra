//! Value and operators in calculation graph

use cauchy::Scalar;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Unary {
    Neg,
    Square,
    Exp,
    Sin,
    Cos,
    Tan,
}

impl Unary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, arg: A) -> A {
        match self {
            Unary::Neg => -arg,
            Unary::Square => arg.conj() * arg,
            Unary::Exp => arg.exp(),
            Unary::Sin => arg.sin(),
            Unary::Cos => arg.cos(),
            Unary::Tan => arg.tan(),
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, arg: A, deriv: A) -> A {
        match self {
            Unary::Neg => -deriv,
            Unary::Square => A::from_f64(2.0).unwrap() * arg.conj() * deriv,
            Unary::Exp => arg.exp() * deriv,
            Unary::Sin => arg.cos() * deriv,
            Unary::Cos => -arg.sin() * deriv,
            Unary::Tan => -deriv / (arg.cos() * arg.cos()),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Binary {
    Add,
    Mul,
    Div,
    Pow,
}

impl Binary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, lhs: A, rhs: A) -> A {
        match self {
            Binary::Add => lhs + rhs,
            Binary::Mul => lhs * rhs,
            Binary::Div => lhs / rhs,
            Binary::Pow => lhs.pow(rhs),
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, lhs: A, rhs: A, deriv: A) -> (A, A) {
        match self {
            Binary::Add => (deriv, deriv),
            Binary::Mul => (rhs * deriv, lhs * deriv),
            Binary::Div => (deriv / rhs, -lhs * deriv / (rhs * rhs)),
            Binary::Pow => (
                rhs * lhs.pow(rhs - A::from_f64(1.0).unwrap()),
                rhs.ln() * lhs.pow(rhs),
            ),
        }
    }
}
