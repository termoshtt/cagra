//! Multiplicate two values

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Mul;

impl<A: Scalar> BinaryOperator<A> for Mul {
    fn eval_value(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>> {
        match (lhs, rhs) {
            (&Value::Scalar(ref l), &Value::Scalar(ref r)) => Ok((*l * *r).into()),
            (&Value::Vector(ref l), &Value::Vector(ref r)) => Ok((l * r).into()),
            (&Value::Matrix(ref l), &Value::Matrix(ref r)) => Ok((l * r).into()),
            _ => Err(BinOpTypeError {}.into()),
        }
    }

    fn eval_deriv(
        &self,
        lhs: &Value<A>,
        rhs: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)> {
        match (lhs, rhs, deriv) {
            (&Value::Scalar(ref l), &Value::Scalar(ref r), &Value::Scalar(ref d)) => {
                Ok(((*r * *d).into(), (*l * *d).into()))
            }
            (&Value::Vector(ref l), &Value::Vector(ref r), &Value::Vector(ref d)) => {
                Ok(((r * d).into(), (l * d).into()))
            }
            (&Value::Matrix(ref l), &Value::Matrix(ref r), &Value::Matrix(ref d)) => {
                Ok(((r * d).into(), (l * d).into()))
            }
            _ => Err(BinOpTypeError {}.into()),
        }
    }
}
