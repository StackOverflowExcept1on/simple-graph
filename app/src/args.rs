use std::path::PathBuf;

#[derive(argh::FromArgs)]
/// Reads a graph from the specified file and applies the search algorithm
/// from the specified start vertex
pub struct Arguments {
    /// path to file in Trivial Graph Format
    #[argh(positional)]
    pub file: PathBuf,
    /// algorithm to process graph (bfs or dfs)
    #[argh(option, from_str_fn(parse_algorithm))]
    pub algorithm: AlgorithmType,
    /// start vertex name in the graph
    #[argh(option)]
    pub start_vertex: String,
}

#[derive(Debug)]
pub enum AlgorithmType {
    BreadthFirstSearch,
    DepthFirstSearch,
}

fn parse_algorithm(value: &str) -> Result<AlgorithmType, String> {
    match value {
        "BFS" | "bfs" => Ok(AlgorithmType::BreadthFirstSearch),
        "DFS" | "dfs" => Ok(AlgorithmType::DepthFirstSearch),
        _ => Err("unknown algorithm type, only bfs and dfs is available".into()),
    }
}
