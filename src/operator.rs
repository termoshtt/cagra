//! Value and operators in calculation graph

use cauchy::Scalar;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Unary {
    Neg,
    Square,
    Exp,
    Ln,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
}

impl Unary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, arg: A) -> A {
        match self {
            Unary::Neg => -arg,
            Unary::Square => arg.conj() * arg,
            Unary::Exp => arg.exp(),
            Unary::Ln => arg.ln(),
            Unary::Sin => arg.sin(),
            Unary::Cos => arg.cos(),
            Unary::Tan => arg.tan(),
            Unary::Sinh => arg.sinh(),
            Unary::Cosh => arg.cosh(),
            Unary::Tanh => arg.tanh(),
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, arg: A, deriv: A) -> A {
        match self {
            Unary::Neg => -deriv,
            Unary::Square => A::from_f64(2.0).unwrap() * arg.conj() * deriv,
            Unary::Exp => arg.exp() * deriv,
            Unary::Ln => deriv / arg,
            Unary::Sin => arg.cos() * deriv,
            Unary::Cos => -arg.sin() * deriv,
            Unary::Tan => -deriv / (arg.cos() * arg.cos()),
            Unary::Sinh => arg.cosh() * deriv,
            Unary::Cosh => arg.sinh() * deriv,
            Unary::Tanh => deriv / (arg.cosh() * arg.cosh()),
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
