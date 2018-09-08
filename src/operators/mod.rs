//! Value and operators in calculation graph

use super::scalar::Field;
use std::fmt::Debug;

pub mod add;
pub mod mul;
pub mod neg;

/// Unary Operators
pub trait UnaryOperator<A: Field>: Clone + Debug {
    /// Evaluate the result value of the operator
    fn eval_value(&self, arg: A) -> A;
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    fn eval_deriv(&self, arg: A, deriv: A) -> A;
}

/// Enumerate of UnaryOperators implementing `UnaryOperator<A>` trait
#[derive(Debug, Clone, Copy, IntoEnum)]
pub enum UnaryOperatorAny {
    Neg(neg::Neg),
}

/// Negate operator
pub fn neg() -> UnaryOperatorAny {
    neg::Neg {}.into()
}

impl<A: Field> UnaryOperator<A> for UnaryOperatorAny {
    fn eval_value(&self, arg: A) -> A {
        match self {
            &UnaryOperatorAny::Neg(op) => op.eval_value(arg),
        }
    }

    fn eval_deriv(&self, arg: A, deriv: A) -> A {
        match self {
            &UnaryOperatorAny::Neg(op) => op.eval_deriv(arg, deriv),
        }
    }
}

/// Binary Operators
pub trait BinaryOperator<A: Field>: Clone + Debug {
    /// Evaluate the result value of the operator
    fn eval_value(&self, lhs: A, rhs: A) -> A;
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    fn eval_deriv(&self, lhs: A, rhs: A, deriv: A) -> (A, A);
}

/// Enumerate of BinaryOperator implementing `BinaryOperator<A>` trait
#[derive(Debug, Clone, Copy, IntoEnum)]
pub enum BinaryOperatorAny {
    Add(add::Add),
    Mul(mul::Mul),
}

/// Add two values
pub fn add() -> BinaryOperatorAny {
    add::Add {}.into()
}

pub fn mul() -> BinaryOperatorAny {
    mul::Mul {}.into()
}

impl<A: Field> BinaryOperator<A> for BinaryOperatorAny {
    fn eval_value(&self, lhs: A, rhs: A) -> A {
        match self {
            &BinaryOperatorAny::Add(op) => op.eval_value(lhs, rhs),
            &BinaryOperatorAny::Mul(op) => op.eval_value(lhs, rhs),
        }
    }

    fn eval_deriv(&self, lhs: A, rhs: A, deriv: A) -> (A, A) {
        match self {
            &BinaryOperatorAny::Add(op) => op.eval_deriv(lhs, rhs, deriv),
            &BinaryOperatorAny::Mul(op) => op.eval_deriv(lhs, rhs, deriv),
        }
    }
}
