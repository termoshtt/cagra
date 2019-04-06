//! Value and operators in calculation graph

use cauchy::Scalar;
use ndarray::{azip, ArrayD};
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
    pub fn eval_value<A: Scalar>(&self, arg: ArrayD<A>) -> ArrayD<A> {
        match self {
            Unary::Neg => -arg,
            Unary::Square => arg.mapv(|a| a.conj() * a),
            Unary::Exp => arg.mapv(|a| a.exp()),
            Unary::Ln => arg.mapv(|a| a.ln()),
            Unary::Sin => arg.mapv(|a| a.sin()),
            Unary::Cos => arg.mapv(|a| a.cos()),
            Unary::Tan => arg.mapv(|a| a.tan()),
            Unary::Sinh => arg.mapv(|a| a.sinh()),
            Unary::Cosh => arg.mapv(|a| a.cosh()),
            Unary::Tanh => arg.mapv(|a| a.tanh()),
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, arg: ArrayD<A>, deriv: ArrayD<A>) -> ArrayD<A> {
        match self {
            Unary::Neg => -deriv,
            Unary::Square => {
                azip!(mut deriv, arg in { *deriv *= A::from_f64(2.0).unwrap() * arg.conj() });
                deriv
            }
            Unary::Exp => {
                azip!(mut deriv, arg in { *deriv *= arg.exp() });
                deriv
            }
            Unary::Ln => {
                azip!(mut deriv, arg in { *deriv /= arg });
                deriv
            }
            Unary::Sin => {
                azip!(mut deriv, arg in { *deriv *= arg.cos() });
                deriv
            }
            Unary::Cos => {
                azip!(mut deriv, arg in { *deriv *= -arg.sin() });
                deriv
            }
            Unary::Tan => {
                azip!(mut deriv, arg in { *deriv /= -arg.cos() * arg.cos() });
                deriv
            }
            Unary::Sinh => {
                azip!(mut deriv, arg in { *deriv *= arg.cosh() });
                deriv
            }
            Unary::Cosh => {
                azip!(mut deriv, arg in { *deriv *= arg.sinh() });
                deriv
            }
            Unary::Tanh => {
                azip!(mut deriv, arg in { *deriv /= arg.cosh() * arg.cosh() });
                deriv
            }
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
    pub fn eval_value<A: Scalar>(&self, lhs: ArrayD<A>, rhs: ArrayD<A>) -> ArrayD<A> {
        match self {
            Binary::Add => lhs + rhs,
            Binary::Mul => lhs * rhs,
            Binary::Div => lhs / rhs,
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(
        &self,
        lhs: ArrayD<A>,
        rhs: ArrayD<A>,
        deriv: ArrayD<A>,
    ) -> (ArrayD<A>, ArrayD<A>) {
        match self {
            Binary::Add => (deriv, deriv),
            Binary::Mul => (rhs * deriv, lhs * deriv),
            Binary::Div => (deriv / rhs, -lhs * deriv / (rhs * rhs)),
        }
    }
}
