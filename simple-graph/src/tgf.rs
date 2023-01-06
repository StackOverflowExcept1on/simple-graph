use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use super::{Graph, ParseGraphError, VertexId};

/// Trait used for serialization and deserialization of the Trivial Graph Format
pub trait Label: Ord + Hash + Clone + Default + Debug + FromStr + Display {}

impl<T: Ord + Hash + Clone + Default + Debug + FromStr + Display> Label for T {}

impl<V: Label, E: Label> Display for Graph<V, E> {
    /// Formats graph as string in Trivial Graph Format
    ///
    /// ```
    /// use simple_graph::Graph;
    ///
    /// let mut graph = Graph::<String, String>::new();
    ///
    /// let first_node_id = graph.add_vertex("First node".into()).unwrap();
    /// let second_node_id = graph.add_vertex("Second node".into()).unwrap();
    ///
    /// graph.add_edge(first_node_id, second_node_id, "Edge between the two".into()).unwrap();
    ///
    /// let s = concat!(
    ///     "1 First node\n",
    ///     "2 Second node\n",
    ///     "#\n",
    ///     "1 2 Edge between the two\n",
    /// );
    /// assert_eq!(graph.to_string(), s);
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut vertices = HashMap::<VertexId, usize>::with_capacity(self.vertices_count());

        for (n, (&vertex_id, _)) in (1_usize..).zip(self.vertices.iter()) {
            if let Ok(vertex_data) = self.get_vertex(vertex_id) {
                vertices.insert(vertex_id, n);
                writeln!(f, "{n} {vertex_data}")?;
            }
        }

        writeln!(f, "#")?;

        for ([from, to], edge) in self.edges().expect("failed to get edges") {
            let from_id = self.get_vertex_id(from);
            let to_id = self.get_vertex_id(to);
            if let Some((from, to)) = vertices.get(&from_id).zip(vertices.get(&to_id)) {
                writeln!(f, "{from} {to} {edge}")?;
            }
        }

        Ok(())
    }
}

/// Trivial Graph Format has two types of definitions which is separated by `#` char
#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum ParserMode {
    /// `1 vertex label`, `<id: usize> <label: &str>`
    VertexDefinitions,
    /// `1 2 edge label`, `<from: usize> <to: usize> <label: &str>`
    EdgeDefinitions,
}

fn parse_index(s: &str, line: usize) -> Result<usize, ParseGraphError> {
    s.parse().map_err(|_| ParseGraphError::ParseInt(line))
}

fn parse_label<T: FromStr>(s: &str, line: usize) -> Result<T, ParseGraphError> {
    s.parse::<T>()
        .map_err(|_| ParseGraphError::ParseLabel(line))
}

impl<V: Label, E: Label> FromStr for Graph<V, E> {
    type Err = ParseGraphError;

    /// Parses [`crate::Graph<V, E>`] from [`&str`] in Trivial Graph Format
    ///
    /// ```
    /// use simple_graph::{Graph, VertexId};
    /// use std::str::FromStr;
    ///
    /// let s = concat!(
    ///     "1 First node\n",
    ///     "2 Second node\n",
    ///     "#\n",
    ///     "1 2 Edge between the two\n",
    /// );
    /// let mut graph = Graph::<String, String>::from_str(s).unwrap();
    ///
    /// let first_node_id = graph.get_vertex_id(&"First node".into());
    /// let second_node_id = graph.get_vertex_id(&"Second node".into());
    ///
    /// let ([from, to], edge) = graph.get_edge(first_node_id, second_node_id).unwrap();
    /// assert!(*from == first_node_id && *to == second_node_id && edge == "Edge between the two");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Self::new();
        let mut vertices = HashMap::<usize, VertexId>::new();

        let mut mode = ParserMode::VertexDefinitions;
        for (n, line) in (1_usize..).zip(s.lines()) {
            if line.starts_with('#') {
                mode = ParserMode::EdgeDefinitions;
                continue;
            }

            let mut it = line.split_whitespace();
            match mode {
                ParserMode::VertexDefinitions => {
                    let s = it.next().ok_or(ParseGraphError::VertexDefinition(n))?;
                    let index = parse_index(s, n)?;
                    let label: V = parse_label(it.remainder().unwrap_or(""), n)?;

                    let vertex_id = graph
                        .add_vertex(label)
                        .map_err(|err| ParseGraphError::GraphError(err, n))?;

                    if vertices.insert(index, vertex_id).is_some() {
                        return Err(ParseGraphError::VertexAlreadyDefined(index, n));
                    }
                }
                ParserMode::EdgeDefinitions => {
                    let (from, to) = it
                        .next()
                        .zip(it.next())
                        .ok_or(ParseGraphError::EdgeDefinition(n))?;

                    let from = parse_index(from, n)?;
                    let to = parse_index(to, n)?;
                    let label: E = parse_label(it.remainder().unwrap_or(""), n)?;

                    let (&from, &to) = vertices
                        .get(&from)
                        .zip(vertices.get(&to))
                        .ok_or(ParseGraphError::VerticesNotDefined(from, to, n))?;

                    graph
                        .add_edge(from, to, label)
                        .map_err(|err| ParseGraphError::GraphError(err, n))?;
                }
            }
        }

        Ok(graph)
    }
}
