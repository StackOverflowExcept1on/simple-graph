### simple-graph

[![Build Status](https://github.com/StackOverflowExcept1on/simple-graph/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/simple-graph/actions)
[![Latest Version](https://img.shields.io/crates/v/simple-graph.svg)](https://crates.io/crates/simple-graph)
[![Documentation](https://docs.rs/simple-graph/badge.svg)](https://docs.rs/simple-graph/)

Graph library with ability to serialize/deserialize Trivial Graph Format

This library implements structure `Graph<V, E>` with adjacency list, **NOT the adjacency matrix**.
This feature allows to optimize memory consumption.

Besides serialize/deserialize library can deal with graph algorithms such
as [Depth-first search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search)
and [Breadth-first search (BFS)](https://en.wikipedia.org/wiki/Breadth-first_search).
You can see the difference here:

![graphAlgorithms](assets/dfs-vs-bfs.gif)

### Code examples?

- visit [documentation](https://docs.rs/simple-graph/)
- you can also check binary that uses this library as dependency
  here: https://github.com/StackOverflowExcept1on/simple-graph/tree/master/app
