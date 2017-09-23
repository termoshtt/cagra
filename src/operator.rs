
use ndarray_linalg::*;

use graph::*;
use error::*;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Negate,
}

impl UnaryOperator {
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
