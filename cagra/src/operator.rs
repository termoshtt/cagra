//! Value and operators in calculation graph

use cauchy::Scalar;
use ndarray::azip;
use serde_derive::{Deserialize, Serialize};

use crate::tensor::*;

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
    pub fn eval_value<A: Scalar>(&self, mut arg: Tensor<A>) -> Tensor<A> {
        match self {
            Unary::Neg => {
                arg = -arg;
            }
            Unary::Square => {
                azip!(mut arg in { *arg = arg.conj() * *arg });
            }
            Unary::Exp => {
                azip!(mut arg in { *arg = arg.exp() });
            }
            Unary::Ln => {
                azip!(mut arg in { *arg = arg.ln() });
            }
            Unary::Sin => {
                azip!(mut arg in { *arg = arg.sin() });
            }
            Unary::Cos => {
                azip!(mut arg in { *arg = arg.cos() });
            }
            Unary::Tan => {
                azip!(mut arg in { *arg = arg.tan() });
            }
            Unary::Sinh => {
                azip!(mut arg in { *arg = arg.sinh() });
            }
            Unary::Cosh => {
                azip!(mut arg in { *arg = arg.cosh() });
            }
            Unary::Tanh => {
                azip!(mut arg in { *arg = arg.tanh() });
            }
        }
        arg
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(&self, arg: Tensor<A>, mut deriv: Tensor<A>) -> Tensor<A> {
        match self {
            Unary::Neg => {
                deriv = -deriv;
            }
            Unary::Square => {
                azip!(mut deriv, arg in { *deriv *= A::from_f64(2.0).unwrap() * arg.conj() });
            }
            Unary::Exp => {
                azip!(mut deriv, arg in { *deriv *= arg.exp() });
            }
            Unary::Ln => {
                azip!(mut deriv, arg in { *deriv /= arg });
            }
            Unary::Sin => {
                azip!(mut deriv, arg in { *deriv *= arg.cos() });
            }
            Unary::Cos => {
                azip!(mut deriv, arg in { *deriv *= -arg.sin() });
            }
            Unary::Tan => {
                azip!(mut deriv, arg in { *deriv /= -arg.cos() * arg.cos() });
            }
            Unary::Sinh => {
                azip!(mut deriv, arg in { *deriv *= arg.cosh() });
            }
            Unary::Cosh => {
                azip!(mut deriv, arg in { *deriv *= arg.sinh() });
            }
            Unary::Tanh => {
                azip!(mut deriv, arg in { *deriv /= arg.cosh() * arg.cosh() });
            }
        }
        deriv
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Binary {
    Add,
    Mul,
    Div,
    Dot,
}

impl Binary {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, lhs: Tensor<A>, rhs: Tensor<A>) -> Tensor<A> {
        match self {
            Binary::Add => lhs + rhs,
            Binary::Mul => lhs * rhs,
            Binary::Div => lhs / rhs,
            Binary::Dot => (lhs * rhs).sum().into_tensor(),
        }
    }
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(
        &self,
        lhs: Tensor<A>,
        rhs: Tensor<A>,
        deriv: Tensor<A>,
    ) -> (Tensor<A>, Tensor<A>) {
        match self {
            Binary::Add => (deriv.clone(), deriv.clone()),
            Binary::Mul => (rhs * deriv.clone(), lhs * deriv.clone()),
            Binary::Div => (
                deriv.clone() / rhs.clone(),
                -lhs * deriv.clone() / (rhs.clone() * rhs.clone()),
            ),
            Binary::Dot => {
                let d = deriv.as_scalar().unwrap();
                (rhs.mapv_into(|a| a * d), lhs.mapv_into(|a| a * d))
            }
        }
    }
}
