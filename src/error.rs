pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    /// node type mismatch
    #[fail(display = "Node type mismatch (Index = {})", index)]
    NodeTypeError { index: usize },

    /// Name duplication in a graph
    #[fail(display = "Duplicated name (name = {})", name)]
    DuplicatedName { name: String },
}
