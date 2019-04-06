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
    pub fn eval_value<A: Scalar>(&self, arg: Tensor<A>) -> Tensor<A> {
        match self {
            Unary::Neg => -arg,
            Unary::Square => arg.mapv_into(|a| a.conj() * a),
            Unary::Ln => arg.mapv_into(|a| a.ln()),
            Unary::Exp => arg.mapv_into(|a| a.exp()),
            Unary::Sin => arg.mapv_into(|a| a.sin()),
            Unary::Cos => arg.mapv_into(|a| a.cos()),
            Unary::Tan => arg.mapv_into(|a| a.tan()),
            Unary::Sinh => arg.mapv_into(|a| a.sinh()),
            Unary::Cosh => arg.mapv_into(|a| a.cosh()),
            Unary::Tanh => arg.mapv_into(|a| a.tanh()),
        }
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
