
use graph::*;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    BinOpTypeError(BinOpTypeError),
}

#[derive(Debug, new)]
pub struct BinOpTypeError {
    op: BinaryOperator,
}
