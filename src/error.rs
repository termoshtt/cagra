//! Errors in cagra

/// abbrev of Result using the master error enum `Error`
pub type Result<T> = ::std::result::Result<T, Error>;

/// Master error enum of cagra
#[derive(Debug, IntoEnum)]
pub enum Error {
    UnaryOpTypeError(UnaryOpTypeError),
    BinOpTypeError(BinOpTypeError),
    CastError(CastError),
    NodeTypeError(NodeTypeError),
}

/// Error that the argument of `UnaryOperator<A>` is invalid
#[derive(Debug)]
pub struct UnaryOpTypeError {}

/// Error that the arguments of `BinaryOperator<A>` is invalid
#[derive(Debug)]
pub struct BinOpTypeError {}

/// Error that the cast of `Value<A>` failed
#[derive(Debug)]
pub struct CastError {}

/// Error that the node type is missing
#[derive(Debug)]
pub struct NodeTypeError {}
