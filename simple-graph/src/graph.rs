use std::collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque};
use std::hash::Hasher;

use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;

use super::{GraphOperationError, Label, Result};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VertexId(u64);

/// Directed graph data-structure with generic parameters
///
/// It is assumed that the user can specify the data types stored at each vertex and edge.
///
/// For example, if you want to make a structure like this:
/// `(town1) <----- 100 km -----> (town2)`
/// you can use [`Graph<String, u32>`] data type
///
/// ### Serialization to [`String`] in Trivial Graph Format
/// See [`impl<V, E> Display for Graph<V, E>`](#impl-Display-for-Graph<V%2C%20E>)
///
/// ### Deserialization from [`&str`] in Trivial Graph Format
/// See [`impl<V, E> FromStr for Graph<V, E>`](#impl-FromStr-for-Graph<V%2C%20E>)
///
/// ### Example
/// In this example, we will make several cities and link them together with specified distance in km
///
/// ```
/// use simple_graph::Graph;
/// use std::str::FromStr;
///
/// let mut graph = Graph::<String, u32>::new();
///
/// let moscow = graph.add_vertex("Moscow".into()).unwrap();
/// let vladimir = graph.add_vertex("Vladimir".into()).unwrap();
/// let yaroslavl = graph.add_vertex("Yaroslavl".into()).unwrap();
/// let novgorod = graph.add_vertex("Novgorod".into()).unwrap();
/// let vologda = graph.add_vertex("Vologda".into()).unwrap();
///
/// graph.add_edge(moscow, vladimir, 180).unwrap();
/// graph.add_edge(moscow, yaroslavl, 250).unwrap();
/// graph.add_edge(vladimir, novgorod, 225).unwrap();
/// graph.add_edge(yaroslavl, vologda, 175).unwrap();
///
/// let serialized = graph.to_string();
/// let expected = concat!(
///     "1 Moscow\n",
///     "2 Vladimir\n",
///     "3 Yaroslavl\n",
///     "4 Novgorod\n",
///     "5 Vologda\n",
///     "#\n",
///     "1 2 180\n",
///     "1 3 250\n",
///     "2 4 225\n",
///     "3 5 175\n",
/// );
/// assert_eq!(serialized, expected);
///
/// let mut graph_deserialized = Graph::from_str(&serialized).unwrap();
/// assert_eq!(graph, graph_deserialized);
/// ```
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Graph<V: Label, E: Label> {
    pub(crate) vertices: LinkedHashMap<VertexId, LinkedHashSet<([VertexId; 2], E)>>,
    pub(crate) vertices_data: HashMap<VertexId, V>,
}

impl<V: Label, E: Label> Graph<V, E> {
    /// Creates new graph
    ///
    /// ```
    /// use simple_graph::Graph;
    ///
    /// let _: Graph<String, u32> = Graph::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets [`VertexId`] by it's value. It just creates hash of the `&V`, doesn't panics
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::new();
    /// assert_ne!(graph.get_vertex_id(&"Moscow".into()), graph.get_vertex_id(&"Vladimir".into()));
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    /// let vladimir = graph.get_vertex_id(&"Vladimir".into());
    ///
    /// assert_eq!(graph.get_edge_value(moscow, vladimir), Ok(&180));
    /// ```
    pub fn get_vertex_id(&self, vertex: &V) -> VertexId {
        let mut hasher = DefaultHasher::new();
        vertex.hash(&mut hasher);
        VertexId(hasher.finish())
    }

    /// Trying to add vertex to the graph, returns [`GraphOperationError::VertexAlreadyExists`]
    /// if can't do this
    ///
    ///
    /// ```
    /// use simple_graph::{Graph, GraphOperationError};
    ///
    /// let mut graph: Graph<String, u32> = Graph::new();
    ///
    /// assert!(graph.add_vertex("Moscow".into()).is_ok());
    /// assert_eq!(graph.add_vertex("Moscow".into()), Err(GraphOperationError::VertexAlreadyExists));
    /// ```
    pub fn add_vertex(&mut self, vertex: V) -> Result<VertexId> {
        let vertex_id = self.get_vertex_id(&vertex);
        if self
            .vertices
            .insert(vertex_id, LinkedHashSet::new())
            .is_some()
        {
            return Err(GraphOperationError::VertexAlreadyExists);
        }
        self.vertices_data.insert(vertex_id, vertex);
        Ok(vertex_id)
    }

    /// Trying to get vertex by id, returns [`GraphOperationError::VertexDoesNotExist`]
    /// if can't do this
    ///
    /// ```
    /// use simple_graph::{Graph, GraphOperationError};
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// assert!(graph.get_vertex(graph.get_vertex_id(&"Moscow".into())).is_ok());
    /// assert_eq!(graph.get_vertex(graph.get_vertex_id(&"New York".into())), Err(GraphOperationError::VertexDoesNotExist));
    /// ```
    pub fn get_vertex(&self, vertex: VertexId) -> Result<&V> {
        self.vertices_data
            .get(&vertex)
            .ok_or(GraphOperationError::VertexDoesNotExist)
    }

    /// Trying to remove vertex by id, returns [`GraphOperationError::VertexDoesNotExist`]
    /// if can't do this
    ///
    /// ```
    /// use simple_graph::{Graph, GraphOperationError};
    /// use std::str::FromStr;
    ///
    /// let mut graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// assert!(graph.remove_vertex(graph.get_vertex_id(&"Moscow".into())).is_ok());
    /// assert_eq!(graph.remove_vertex(graph.get_vertex_id(&"New York".into())), Err(GraphOperationError::VertexDoesNotExist));
    ///
    /// assert_eq!(graph.vertices_count(), 4);
    /// ```
    pub fn remove_vertex(&mut self, target_vertex: VertexId) -> Result<()> {
        self.get_vertex(target_vertex)?;

        let mut pairs = Vec::new();
        for &other_vertex in self.vertices.keys() {
            if let Ok(edge) = self.get_edge(other_vertex, target_vertex).cloned() {
                pairs.push((other_vertex, edge));
            }
        }

        for (other_vertex, edge) in &pairs {
            if let Some(neighbours) = self.vertices.get_mut(other_vertex) {
                neighbours.remove(edge);
            }
        }

        self.vertices.remove(&target_vertex);
        self.vertices_data.remove(&target_vertex);

        Ok(())
    }

    /// Trying to add edge between two vertices, returns [`GraphOperationError::VertexDoesNotExist`]
    /// if can't do this
    ///
    /// ```
    /// use simple_graph::{Graph, GraphOperationError};
    /// use std::str::FromStr;
    ///
    /// let mut graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let novgorod = graph.get_vertex_id(&"Novgorod".into());
    /// let kazan = graph.add_vertex("Kazan".into()).unwrap();
    /// let new_york = graph.get_vertex_id(&"New York".into());
    ///
    /// assert!(graph.add_edge(novgorod, kazan, 325).is_ok());
    /// assert_eq!(graph.add_edge(new_york, kazan, 9000), Err(GraphOperationError::VertexDoesNotExist));
    /// ```
    pub fn add_edge(&mut self, from: VertexId, to: VertexId, edge: E) -> Result<()> {
        if self.vertices.get(&to).is_some() {
            if let Some(neighbours) = self.vertices.get_mut(&from) {
                neighbours.insert(([from, to], edge));
                return Ok(());
            }
        }
        Err(GraphOperationError::VertexDoesNotExist)
    }

    /// Trying to get edge between two vertices, returns [`GraphOperationError::EdgeDoesNotExist`]
    /// if can't do this
    ///
    /// ```
    /// use simple_graph::{Graph, GraphOperationError};
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    /// let vladimir = graph.get_vertex_id(&"Vladimir".into());
    ///
    /// // Moscow -> Vladimir (ok)
    /// let ([_from, _to], value) = graph.get_edge(moscow, vladimir).unwrap();
    /// assert_eq!(value, &180);
    /// // Vladimir -> Moscow (err)
    /// assert_eq!(graph.get_edge(vladimir, moscow), Err(GraphOperationError::EdgeDoesNotExist));
    /// ```
    pub fn get_edge(&self, from: VertexId, to: VertexId) -> Result<&([VertexId; 2], E)> {
        if let Some(neighbours) = self.vertices.get(&from) {
            for edge in neighbours {
                let [_, destination_vertex] = edge.0;
                if destination_vertex == to {
                    return Ok(edge);
                }
            }
        }
        Err(GraphOperationError::EdgeDoesNotExist)
    }

    /// Trying to get edge **value** between two vertices, returns [`GraphOperationError::EdgeDoesNotExist`]
    /// if can't do this
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    /// let vladimir = graph.get_vertex_id(&"Vladimir".into());
    ///
    /// assert_eq!(graph.get_edge_value(moscow, vladimir), Ok(&180));
    /// ```
    pub fn get_edge_value(&self, from: VertexId, to: VertexId) -> Result<&E> {
        let (_, value) = self.get_edge(from, to)?;
        Ok(value)
    }

    /// Trying to remove edge between two vertices, returns [`GraphOperationError::EdgeDoesNotExist`]
    /// if can't do this
    ///
    /// ```
    /// use simple_graph::{Graph, GraphOperationError};
    /// use std::str::FromStr;
    ///
    /// let mut  graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    /// let vladimir = graph.get_vertex_id(&"Vladimir".into());
    ///
    /// assert!(graph.remove_edge(moscow, vladimir).is_ok());
    /// assert_eq!(graph.remove_edge(moscow, vladimir), Err(GraphOperationError::EdgeDoesNotExist));
    /// ```
    pub fn remove_edge(&mut self, from: VertexId, to: VertexId) -> Result<()> {
        if let Ok(edge) = self.get_edge(from, to).cloned() {
            if let Some(neighbours) = self.vertices.get_mut(&from) {
                neighbours.remove(&edge);
                return Ok(());
            }
        }
        Err(GraphOperationError::EdgeDoesNotExist)
    }

    /// Returns count of vertices in the graph
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let mut graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    /// assert_eq!(graph.vertices_count(), 5);
    /// ```
    pub fn vertices_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns count of edges in the graph
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let mut graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    /// assert_eq!(graph.edges_count(), 4);
    /// ```
    pub fn edges_count(&self) -> usize {
        let mut count = 0;
        for (_, neighbours) in self.vertices.iter() {
            count += neighbours.len();
        }
        count
    }

    /// Returns [`Result<Vec<(&V, Vec<(&V, &E)>)>>`] which is vertices representation
    ///
    /// where
    /// - `&V` - vertex
    /// - `Vec<(&V, &E)>` - adjacent vertices
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// for (vertex, adjacent_vertices) in graph.vertices().unwrap() {
    ///     println!("{vertex}: {adjacent_vertices:?}");
    /// }
    ///
    /// //output looks like
    ///
    /// //Moscow: [("Vladimir", 180), ("Yaroslavl", 250)]
    /// //Vladimir: [("Novgorod", 225)]
    /// //Yaroslavl: [("Vologda", 175)]
    /// //Novgorod: []
    /// //Vologda: []
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn vertices(&self) -> Result<Vec<(&V, Vec<(&V, &E)>)>> {
        self.vertices
            .keys()
            .map(|&id| self.get_vertex_info(id))
            .collect::<Result<Vec<_>>>()
    }

    /// Returns [`Result<Vec<([&V; 2], &E)>>`] which is edges representation
    ///
    /// where
    /// - `[&V; 2]` - `[from, to]`
    /// - `&E` - edge data
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// for ([from, to], data) in graph.edges().unwrap() {
    ///    println!("{from} ---- [{data}] ----> {to}");
    /// }
    ///
    /// //output looks like
    ///
    /// //Moscow ---- [180] ----> Vladimir
    /// //Moscow ---- [250] ----> Yaroslavl
    /// //Vladimir ---- [225] ----> Novgorod
    /// //Yaroslavl ---- [175] ----> Vologda
    /// ```
    pub fn edges(&self) -> Result<Vec<([&V; 2], &E)>> {
        let mut edges = Vec::new();
        for (_, neighbours) in self.vertices.iter() {
            for ([from, to], edge) in neighbours {
                edges.push(([self.get_vertex(*from)?, self.get_vertex(*to)?], edge));
            }
        }
        Ok(edges)
    }

    /// Trying to get detailed information about the vertex
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> = Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    ///
    /// let (vertex_data, adjacent_vertices) = graph.get_vertex_info(moscow).unwrap();
    /// assert_eq!(vertex_data, &String::from("Moscow"));
    /// assert_eq!(adjacent_vertices, vec![(&"Vladimir".into(), &180), (&"Yaroslavl".into(), &250)]);
    /// ```
    pub fn get_vertex_info(&self, vertex_id: VertexId) -> Result<(&V, Vec<(&V, &E)>)> {
        let vertex = self.get_vertex(vertex_id)?;

        let mut adjacent_vertices = Vec::new();
        if let Some(neighbours) = self.vertices.get(&vertex_id) {
            for ([_, vertex], edge) in neighbours {
                adjacent_vertices.push((self.get_vertex(*vertex)?, edge));
            }
        }

        Ok((vertex, adjacent_vertices))
    }

    /// Generic search algorithm for implementation BFS and DFS.
    /// It uses generic queue type and queue operations to prevent code duplication
    fn generic_search<
        QueueTy,
        QueueInitFn: Fn() -> QueueTy,
        QueueInsertFn: Fn(&mut QueueTy, VertexId),
        QueueRemoveFn: Fn(&mut QueueTy) -> Option<VertexId>,
        AccessFn: FnMut(&V, Vec<(&V, &E)>),
    >(
        &self,
        queue_init: QueueInitFn,
        queue_insert: QueueInsertFn,
        queue_remove: QueueRemoveFn,
        source: VertexId,
        mut access: AccessFn,
    ) -> Result<()> {
        let mut queue = queue_init();
        let mut visited = HashSet::new();

        queue_insert(&mut queue, source);
        visited.insert(source);

        while let Some(vertex) = queue_remove(&mut queue) {
            let (vertex, adjacent_vertices) = self.get_vertex_info(vertex)?;
            for (adjacent_vertex, _) in adjacent_vertices.iter() {
                let id = self.get_vertex_id(adjacent_vertex);
                if !visited.contains(&id) {
                    queue_insert(&mut queue, id);
                    visited.insert(id);
                }
            }
            access(vertex, adjacent_vertices);
        }

        Ok(())
    }

    /// Breadth-first search algorithm uses [`VecDeque`] for queue
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> =
    ///     Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    ///
    /// let mut visited = Vec::new();
    /// graph
    ///     .bfs(moscow, |vertex, adjacent_vertices| {
    ///         visited.push(format!("{vertex}: {adjacent_vertices:?}"));
    ///     })
    ///     .expect("failed to perform BFS");
    ///
    /// //(1) Moscow ->
    /// //              (2) Vladimir  -> (4) Novgorod
    /// //              (3) Yaroslavl -> (5) Vologda
    /// let expected = vec![
    ///     r#"Moscow: [("Vladimir", 180), ("Yaroslavl", 250)]"#, //1
    ///     r#"Vladimir: [("Novgorod", 225)]"#, //2
    ///     r#"Yaroslavl: [("Vologda", 175)]"#, //3
    ///     r#"Novgorod: []"#, //4
    ///     r#"Vologda: []"#, //5
    /// ];
    /// assert_eq!(visited, expected);
    /// ```
    pub fn bfs<F: FnMut(&V, Vec<(&V, &E)>)>(&self, source: VertexId, access: F) -> Result<()> {
        self.generic_search(
            VecDeque::new,
            |queue, vertex| queue.push_back(vertex),
            |queue| queue.pop_front(),
            source,
            access,
        )
    }

    /// Depth-first search algorithm uses [`Vec`] for stack
    ///
    /// ```
    /// use simple_graph::Graph;
    /// use std::str::FromStr;
    ///
    /// let graph: Graph<String, u32> =
    ///     Graph::from_str(include_str!("../test_input/moscow.tgf")).unwrap();
    ///
    /// let moscow = graph.get_vertex_id(&"Moscow".into());
    ///
    /// let mut visited = Vec::new();
    /// graph
    ///     .dfs(moscow, |vertex, adjacent_vertices| {
    ///         visited.push(format!("{vertex}: {adjacent_vertices:?}"));
    ///     })
    ///     .expect("failed to perform DFS");
    ///
    /// //(1) Moscow ->
    /// //              (2) Yaroslavl  -> (3) Vologda
    /// //              (4) Vladimir   -> (5) Novgorod
    /// let expected = vec![
    ///     r#"Moscow: [("Vladimir", 180), ("Yaroslavl", 250)]"#,
    ///     r#"Yaroslavl: [("Vologda", 175)]"#,
    ///     r#"Vologda: []"#,
    ///     r#"Vladimir: [("Novgorod", 225)]"#,
    ///     r#"Novgorod: []"#,
    /// ];
    /// assert_eq!(visited, expected);
    /// ```
    pub fn dfs<F: FnMut(&V, Vec<(&V, &E)>)>(&self, source: VertexId, access: F) -> Result<()> {
        self.generic_search(
            Vec::new,
            |stack, vertex| stack.push(vertex),
            |stack| stack.pop(),
            source,
            access,
        )
    }
}
