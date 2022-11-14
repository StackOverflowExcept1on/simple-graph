use std::fs;
use std::str::FromStr;

use simple_graph::Graph;

use colored::*;

pub use args::*;
pub use error::*;

mod args;
mod error;

fn app() -> Result<()> {
    let Arguments {
        file,
        algorithm,
        start_vertex,
    } = argh::from_env();

    let content = fs::read_to_string(file)?;
    let graph: Graph<String, String> = Graph::from_str(&content)?;

    let vertex_id = graph.get_vertex_id(&start_vertex);
    let _ = graph.get_vertex(vertex_id)?; //try to find this in graph first

    println!("{}", "Vertices traversal...".to_string().bright_green());
    for (vertex, adjacent_vertices) in graph.vertices()? {
        println!("- {}", vertex.to_string().bright_yellow());
        for adjacent_vertex in adjacent_vertices {
            println!("  {}", format!("{adjacent_vertex:?}").bright_cyan());
        }
    }

    println!(
        "{}",
        format!("Applying \"{algorithm:?}\" algorithm to this graph...").bright_green()
    );

    let mut visited = Vec::new();
    let visitor_fn = |vertex: &String, adjacent_vertices: Vec<(&String, &String)>| {
        visited.push(format!(
            "{}: {}",
            vertex.to_string().bright_yellow(),
            format!("{adjacent_vertices:?}").bright_cyan()
        ));
    };

    match algorithm {
        AlgorithmType::BreadthFirstSearch => graph.bfs(vertex_id, visitor_fn),
        AlgorithmType::DepthFirstSearch => graph.dfs(vertex_id, visitor_fn),
    }?;

    for (i, line) in (1_usize..).zip(visited) {
        println!("{i}. {line}");
    }

    Ok(())
}

fn main() {
    if let Err(err) = app() {
        println!("Error: {err}");
    }
}
