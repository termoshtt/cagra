//! Errors in cagra

/// abbrev of Result using the master error enum `Error`
pub type Result<T> = ::std::result::Result<T, Error>;

/// Master error enum of cagra
#[derive(Fail, Debug)]
pub enum Error {
    /// the argument of `UnaryOperator<A>` is invalid
    #[fail(display = "vom")]
    UnaryOpTypeError {},
    /// Error that the arguments of `BinaryOperator<A>` is invalid
    #[fail(display = "message!!")]
    BinOpTypeError {},
    /// Error that the cast of `Value<A>` failed
    #[fail(display = "Message")]
    CastError {},
    /// Error that the node type is missing
    #[fail(display = "Message!")]
    NodeTypeError {},
}
