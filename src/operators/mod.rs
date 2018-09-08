//! Value and operators in calculation graph

use super::scalar::Field;
use std::fmt::Debug;
use std::rc::Rc;

mod add;
mod mul;
mod neg;

pub use self::add::add;
pub use self::mul::mul;
pub use self::neg::neg;

pub type UnaryOp<A> = Rc<dyn UnaryOperator<A>>;
pub type BinOp<A> = Rc<dyn BinaryOperator<A>>;

/// Unary Operators
pub trait UnaryOperator<A: Field>: Debug {
    /// Evaluate the result value of the operator
    fn eval_value(&self, arg: A) -> A;
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    fn eval_deriv(&self, arg: A, deriv: A) -> A;
}

/// Binary Operators
pub trait BinaryOperator<A: Field>: Debug {
    /// Evaluate the result value of the operator
    fn eval_value(&self, lhs: A, rhs: A) -> A;
    /// Evaluate the derivative of the operator multiplied by the received
    /// derivative from upper of the graph.
    fn eval_deriv(&self, lhs: A, rhs: A, deriv: A) -> (A, A);
}
