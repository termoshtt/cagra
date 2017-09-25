
use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ScalarMul;

impl<A: Scalar> BinaryOperator<A> for ScalarMul {
    fn eval_value(&self, lhs: &Value<A>, rhs: &Value<A>) -> Result<Value<A>> {
        match (lhs, rhs) {
            (&Value::Scalar(ref a), &Value::Scalar(ref x)) => Ok((*a * *x).into()),
            (&Value::Scalar(ref a), &Value::Vector(ref x)) |
            (&Value::Vector(ref x), &Value::Scalar(ref a)) => Ok((x.mapv(|x| x * *a)).into()),
            (&Value::Scalar(ref a), &Value::Matrix(ref x)) |
            (&Value::Matrix(ref x), &Value::Scalar(ref a)) => Ok((x.mapv(|x| x * *a)).into()),
            _ => Err(BinOpTypeError {}.into()),
        }
    }

    fn eval_deriv(
        &self,
        lhs: &Value<A>,
        rhs: &Value<A>,
        deriv: &Value<A>,
    ) -> Result<(Value<A>, Value<A>)> {
        match (lhs, rhs) {
            (&Value::Vector(_), &Value::Vector(_)) |
            (&Value::Vector(_), &Value::Matrix(_)) |
            (&Value::Matrix(_), &Value::Vector(_)) |
            (&Value::Matrix(_), &Value::Matrix(_)) => Err(BinOpTypeError {}.into()),
            _ => Ok((rhs.clone(), lhs.clone())),
        }
    }
}
