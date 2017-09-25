//! Value and operators in calculation graph

use std::fmt::Debug;
use ndarray::*;
use ndarray_linalg::*;

use error::*;

pub mod neg;
pub mod add;
pub mod scalar_mul;

/// Value in graph
///
/// Each node has a value in the following types:
///
/// - Scalar
/// - Vector (represented by `Array1<A>`)
/// - Matrix (represented by `Array2<A>`)
///
/// These types are selected dynamically, and operators returns `Err`
/// if it does not support the received value.
#[derive(Debug, Clone, IntoEnum)]
pub enum Value<A: Scalar> {
    Scalar(A),
    Vector(Array1<A>),
    Matrix(Array2<A>),
}

impl<A: Scalar> Value<A> {
    /// Return value if it is a scalar
    pub fn as_scalar(&self) -> Result<A> {
        match *self {
            Value::Scalar(a) => Ok(a),
            _ => Err(CastError {}.into()),
        }
    }

    pub fn identity() -> Self {
        Value::Scalar(A::from_f64(1.0))
    }
}

/// Unary Operators
pub trait UnaryOperator<A: Scalar>: Clone + Debug {
    /// Evaluate the result value of the operator
    fn eval_value(&self, arg: &Value<A>) -> Result<Value<A>>;
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    fn eval_deriv(&self, arg: &Value<A>, deriv: &Value<A>) -> Result<Value<A>>;
}

/// Enumerate of UnaryOperators implementing `UnaryOperator<A>` trait
#[derive(Debug, Clone, Copy, IntoEnum)]
pub enum UnaryOperatorAny {
    Neg(neg::Neg),
}

/// Negate operator
pub fn neg() -> UnaryOperatorAny {
    neg::Neg {}.into()
}

impl<A: Scalar> UnaryOperator<A> for UnaryOperatorAny {
    fn eval_value(&self, arg: &Value<A>) -> Result<Value<A>> {
        match self {
            &UnaryOperatorAny::Neg(op) => op.eval_value(arg),
        }
    }

    fn eval_deriv(&self, arg: &Value<A>, deriv: &Value<A>) -> Result<Value<A>> {
        match self {
            &UnaryOperatorAny::Neg(op) => op.eval_deriv(arg, deriv),
        }
    }
}

/// Binary Operators
pub trait BinaryOperator<A: Scalar>: Clone + Debug {
    /// Evaluate the result value of the operator
    fn eval_value(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>>;
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    fn eval_deriv(
        &self,
        lhs: &Value<A>,
        rhs: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)>;
}

/// Enumerate of BinaryOperator implementing `BinaryOperator<A>` trait
#[derive(Debug, Clone, Copy, IntoEnum)]
pub enum BinaryOperatorAny {
    Add(add::Add),
}

/// Add two values
#[derive(Debug, Clone, Copy, IntoEnum)]
pub enum BinaryOperatorAny {
    Add(add::Add),
    ScalarMul(scalar_mul::ScalarMul),
}

pub fn add() -> BinaryOperatorAny {
    add::Add {}.into()
}

pub fn scalar_mul() -> BinaryOperatorAny {
    scalar_mul::ScalarMul {}.into()
}

impl<A: Scalar> BinaryOperator<A> for BinaryOperatorAny {
    fn eval_value(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>> {
        match self {
            &BinaryOperatorAny::Add(op) => op.eval_value(lhs, rhs),
            &BinaryOperatorAny::ScalarMul(op) => op.eval_value(lhs, rhs),
        }
    }

    fn eval_deriv(
        &self,
        lhs: &Value<A>,
        rhs: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)> {
        match self {
            &BinaryOperatorAny::Add(op) => op.eval_deriv(lhs, rhs, deriv),
            &BinaryOperatorAny::ScalarMul(op) => op.eval_deriv(lhs, rhs, deriv),
        }
    }
}
