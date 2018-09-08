//! Negate value

use super::*;
use scalar::Field;

/// Negate
#[derive(Debug, Clone, Copy)]
pub struct Neg;

impl<A: Field> UnaryOperator<A> for Neg {
    fn eval_value(&self, arg: A) -> A {
        -arg
    }

    fn eval_deriv(&self, _arg: A, deriv: A) -> A {
        -deriv
    }
}
