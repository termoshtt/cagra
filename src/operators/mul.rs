//! Multiplicate two values

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Mul;

impl<A: Field> BinaryOperator<A> for Mul {
    fn eval_value(&self, lhs: A, rhs: A) -> A {
        lhs * rhs
    }

    fn eval_deriv(&self, lhs: A, rhs: A, deriv: A) -> (A, A) {
        (rhs * deriv, lhs * deriv)
    }
}
