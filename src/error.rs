
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, IntoEnum)]
pub enum Error {
    ArgTypeMismatch(ArgTypeMismatchError),
}

#[derive(Debug)]
pub struct ArgTypeMismatchError {}
