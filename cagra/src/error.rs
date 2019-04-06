use failure::Fail;
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    /// value node is not initialized
    #[fail(display = "Value node is not initialized (Index = {})", index)]
    ValueUninitialized { index: usize },

    /// derivative is not initialized
    #[fail(display = "Derivative is not initialized (Index = {})", index)]
    DerivUninitialized { index: usize },

    /// node type mismatch
    #[fail(display = "Node type mismatch (Index = {})", index)]
    NodeTypeError { index: usize },

    /// Name duplication in a graph
    #[fail(display = "Duplicated name (name = {})", name)]
    DuplicatedName { name: String },

    /// Name is not defined
    #[fail(display = "Variable name is not defined (name = {})", name)]
    UndefinedName { name: String },

    /// Fail to serialize to JSON
    #[fail(display = "JSON serialization failed: {:?})", error)]
    JSONSerializeFailed { error: serde_json::error::Error },

    /// Tensor rank mismatch
    #[fail(
        display = "Tensor rank is mismatched: actual={}, desired={}",
        actual, desired
    )]
    TensorRankMismatch { actual: usize, desired: usize },
}
