
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    UnaryOpTypeError(UnaryOpTypeError),
    BinOpTypeError(BinOpTypeError),
    CastError(CastError),
    NodeTypeError(NodeTypeError),
}

#[derive(Debug)]
pub struct UnaryOpTypeError {}

#[derive(Debug)]
pub struct BinOpTypeError {}

#[derive(Debug)]
pub struct CastError {}

#[derive(Debug)]
pub struct NodeTypeError {}
