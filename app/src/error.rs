use std::io;

pub type Result<T, E = MyError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
    #[error("graph library error: {0}")]
    GraphOperation(#[from] simple_graph::GraphOperationError),
    #[error("graph parse error: {0}")]
    GraphParse(#[from] simple_graph::ParseGraphError),
}
