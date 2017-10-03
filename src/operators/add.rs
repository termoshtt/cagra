//! Add two values

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Add;

impl<A: Scalar> BinaryOperator<A> for Add {
    fn eval_value(&self, lhs: A, rhs: A) -> A {
        lhs + rhs
    }

    fn eval_deriv(&self, _lhs: A, _rhs: A, deriv: A) -> (A, A) {
        (deriv, deriv)
    }
}
