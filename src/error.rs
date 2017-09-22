
use graph::*;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    BinOpTypeError(BinOpTypeError),
}

#[derive(Debug)]
pub struct BinOpTypeError {
    pub(crate) op: BinaryOperator,
}
