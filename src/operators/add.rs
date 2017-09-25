
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Add;

impl<A: Scalar> BinaryOperator<A> for Add {
    fn eval_value(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>> {
        match (lhs, rhs) {
            (&Value::Scalar(ref l), &Value::Scalar(ref r)) => Ok((*l + *r).into()),
            (&Value::Vector(ref l), &Value::Vector(ref r)) => Ok((l + r).into()),
            (&Value::Matrix(ref l), &Value::Matrix(ref r)) => Ok((l + r).into()),
            _ => Err(BinOpTypeError {}.into()),
        }
    }

    fn eval_deriv(
        &self,
        _lhs: &Value<A>,
        _rhs: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)> {
        Ok((deriv.clone(), deriv.clone()))
    }
}
