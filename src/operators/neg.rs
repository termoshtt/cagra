//! Negate value

use ndarray_linalg::*;

use super::*;

/// Negate
#[derive(Debug, Clone, Copy)]
pub struct Neg;

impl<A: Scalar> UnaryOperator<A> for Neg {
    fn eval_value(&self, arg: A) -> A {
        -arg
    }

    fn eval_deriv(&self, _arg: A, deriv: A) -> A {
        -deriv
    }
}
