//! Negate value

use ndarray_linalg::*;

use super::*;

/// Negate
#[derive(Debug, Clone, Copy)]
pub struct Neg;

impl<A: Scalar> UnaryOperator<A> for Neg {
    fn eval_value(&self, arg: &Value<A>) -> Result<Value<A>> {
        match arg {
            &Value::Scalar(a) => Ok((-a).into()),
            &Value::Vector(ref a) => Ok((-a.to_owned()).into()),
            &Value::Matrix(ref a) => Ok((-a.to_owned()).into()),
        }

    }

    fn eval_deriv(&self, _arg: &Value<A>, deriv: &Value<A>) -> Result<Value<A>> {
        match deriv {
            &Value::Scalar(a) => Ok((-a).into()),
            &Value::Vector(ref a) => Ok((-a.to_owned()).into()),
            &Value::Matrix(ref a) => Ok((-a.to_owned()).into()),
        }
    }
}
