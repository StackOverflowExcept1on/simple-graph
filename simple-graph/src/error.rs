/// Custom Result type with two generic parameters for user convenience
pub type Result<T, E = GraphOperationError> = std::result::Result<T, E>;

/// Describes possible errors that might happen when user interacts with graph
#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum GraphOperationError {
    /// when user trying to create vertex with the same data
    #[error("this vertex_id already exists in the graph")]
    VertexAlreadyExists,
    /// when user trying to get/remove vertex by id and it doesn't exist
    #[error("this vertex_id does not exist in the graph")]
    VertexDoesNotExist,
    /// when user trying to find edge by two vertices and it's failed
    #[error("unable to find edge in graph between two vertices")]
    EdgeDoesNotExist,
}

/// Describes possible errors that might happen during parsing the Trivial Graph Format
#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum ParseGraphError {
    /// `(line: usize)`
    #[error("incorrect vertex definition at line {0}")]
    VertexDefinition(usize),
    /// `(line: usize)`
    #[error("incorrect edge definition at line {0}")]
    EdgeDefinition(usize),

    #[error("vertex with index {0} already defined, check line {1}")]
    /// `(vertex_index: usize, line: usize)`
    VertexAlreadyDefined(usize, usize),
    /// `(from_index: usize, to_index: usize, line: usize)`
    #[error("failed to join vertices with ids {0}, {1} because they are not defined at line {2}")]
    VerticesNotDefined(usize, usize, usize),

    /// `(line: usize)`
    #[error("failed to parse index of the vertex or edge at line {0}")]
    ParseInt(usize),
    /// `(line: usize)`
    #[error("failed to parse label data of the vertex or edge at line {0}")]
    ParseLabel(usize),

    /// internal error with graphs API
    #[error("some graph operation failed: {0} at line {1}")]
    GraphError(GraphOperationError, usize),
}
