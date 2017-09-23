//! Value and operators in calculation graph

use ndarray::*;
use ndarray_linalg::*;

use error::*;

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
#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
    /// Evaluate the result value of the operator
    pub fn eval_value<A: Scalar>(&self, arg: &Value<A>) -> Result<Value<A>> {
        match self {
            &UnaryOperator::Negate => {
                match arg {
                    &Value::Scalar(a) => Ok((-a).into()),
                    &Value::Vector(ref a) => Ok((-a.to_owned()).into()),
                    &Value::Matrix(ref a) => Ok((-a.to_owned()).into()),
                }
            }
        }

    }

    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    pub fn eval_deriv<A: Scalar>(
        &self,
        _arg_last: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<Value<A>> {
        match self {
            &UnaryOperator::Negate => {
                match deriv {
                    &Value::Scalar(a) => Ok((-a).into()),
                    &Value::Vector(ref a) => Ok((-a.to_owned()).into()),
                    &Value::Matrix(ref a) => Ok((-a.to_owned()).into()),
                }
            }
        }
    }
}

/// Binary Operators
#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
}

impl BinaryOperator {
    pub fn eval_value<A: Scalar>(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>> {
        match self {
            &BinaryOperator::Plus => {
                match (lhs, rhs) {
                    (&Value::Scalar(ref l), &Value::Scalar(ref r)) => Ok((*l + *r).into()),
                    (&Value::Vector(ref l), &Value::Vector(ref r)) => Ok((l + r).into()),
                    (&Value::Matrix(ref l), &Value::Matrix(ref r)) => Ok((l + r).into()),
                    _ => Err(BinOpTypeError { op: *self }.into()),
                }
            }
        }
    }

    pub fn eval_deriv<A: Scalar>(
        &self,
        _lhs_last: &Value<A>,
        _rhs_last: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)> {
        match self {
            &BinaryOperator::Plus => Ok((deriv.clone(), deriv.clone())),
        }
    }
}
