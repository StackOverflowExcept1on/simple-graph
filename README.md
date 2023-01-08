### simple-graph

[![Build Status](https://github.com/StackOverflowExcept1on/simple-graph/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/simple-graph/actions)
[![Latest Version](https://img.shields.io/crates/v/simple-graph.svg)](https://crates.io/crates/simple-graph)
[![Documentation](https://docs.rs/simple-graph/badge.svg)](https://docs.rs/simple-graph/)

Graph library with ability to serialize/deserialize Trivial Graph Format, see [simple-graph](simple-graph) directory

### What is this?

This is my test task for `{some_company_name}`

The requirements are listed in the file [REQUIREMENTS-ru.md](REQUIREMENTS-ru.md)

### Installing library from [crates.io](https://crates.io)

```toml
[dependencies]
simple-graph = "0.1.0"
```

### Examples

Go to the [`Graph<V, E>` on docs.rs/simple-graph](https://docs.rs/simple-graph/latest/simple_graph/struct.Graph.html)
for online documentation

I wrote a perfect documentation for all files in code using the `///` rust built-in feature

You can also read [`simple-graph/src/graph.rs`](simple-graph/src/graph.rs)

### Implemented graph algorithms in the binary [app](app)

![graphAlgorithms](simple-graph/assets/dfs-vs-bfs.gif)

#### Breadth-first search algorithm (BFS)

```bash
cargo run -- simple-graph/test_input/moscow.tgf --algorithm bfs --start-vertex Moscow
```

```
Vertices traversal...
- Moscow
  ("Vladimir", "180")
  ("Yaroslavl", "250")
- Vladimir
  ("Novgorod", "225")
- Yaroslavl
  ("Vologda", "175")
- Novgorod
- Vologda
Applying "BreadthFirstSearch" algorithm to this graph...
1. Moscow: [("Vladimir", "180"), ("Yaroslavl", "250")]
2. Vladimir: [("Novgorod", "225")]
3. Yaroslavl: [("Vologda", "175")]
4. Novgorod: []
5. Vologda: []
```

#### Depth-first search algorithm (DFS)

```bash
cargo run -- simple-graph/test_input/moscow.tgf --algorithm dfs --start-vertex Moscow
```

```
Vertices traversal...
- Moscow
  ("Vladimir", "180")
  ("Yaroslavl", "250")
- Vladimir
  ("Novgorod", "225")
- Yaroslavl
  ("Vologda", "175")
- Novgorod
- Vologda
Applying "DepthFirstSearch" algorithm to this graph...
1. Moscow: [("Vladimir", "180"), ("Yaroslavl", "250")]
2. Yaroslavl: [("Vologda", "175")]
3. Vologda: []
4. Vladimir: [("Novgorod", "225")]
5. Novgorod: []
```

### Tests

Rust has 3 types of the test:

- unit-tests
  ```rust
  #[cfg(test)]
  mod tests { }
  ```
- integration tests stored in `crate_name/tests/*.rs`
- **documentation tests** runs from comments near the function/method itself
  ```rust
  /// function foo
  /// 
  /// ```
  /// assert_eq!(foo(), true);
  /// ```
  fn foo() -> bool { true }
  ```

I decided to use last type of the tests to verify code right in the comments to the methods

```bash
cargo test
```

```
   Compiling simple-graph v0.1.0 (/mnt/tmpfs/simple-graph/simple-graph)
   Compiling app v0.1.0 (/mnt/tmpfs/simple-graph/app)
    Finished test [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/main.rs (target/debug/deps/app-bc92a2797886b115)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/simple_graph-5c5fa28164719bd4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests simple-graph

running 19 tests
test src/graph.rs - graph::Graph<V,E>::add_vertex (line 108) ... ok
test src/graph.rs - graph::Graph<V,E>::add_edge (line 186) ... ok
test src/graph.rs - graph::Graph<V,E>::edges (line 351) ... ok
test src/graph.rs - graph::Graph<V,E>::dfs (line 484) ... ok
test src/graph.rs - graph::Graph<V,E>::bfs (line 444) ... ok
test src/graph.rs - graph::Graph (line 29) ... ok
test src/graph.rs - graph::Graph<V,E>::get_vertex_id (line 84) ... ok
test src/graph.rs - graph::Graph<V,E>::get_edge (line 212) ... ok
test src/graph.rs - graph::Graph<V,E>::get_vertex_info (line 380) ... ok
test src/graph.rs - graph::Graph<V,E>::edges_count (line 298) ... ok
test src/graph.rs - graph::Graph<V,E>::get_edge_value (line 242) ... ok
test src/graph.rs - graph::Graph<V,E>::get_vertex (line 132) ... ok
test src/graph.rs - graph::Graph<V,E>::new (line 73) ... ok
test src/graph.rs - graph::Graph<V,E>::vertices_count (line 285) ... ok
test src/graph.rs - graph::Graph<V,E>::remove_edge (line 261) ... ok
test src/graph.rs - graph::Graph<V,E>::remove_vertex (line 150) ... ok
test src/tgf.rs - tgf::Graph<V,E>::fmt (line 16) ... ok
test src/graph.rs - graph::Graph<V,E>::vertices (line 319) ... ok
test src/tgf.rs - tgf::Graph<V,E>::from_str (line 81) ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.49s
```

### Clippy checks

```bash
cargo clippy --all-targets -- -D warnings
```

### Interaction with GitHub Actions

This repository also has a config [`.github/workflows/ci.yml`](.github/workflows/ci.yml) to check all
things that I described above automatically on GitHub servers

### [`Cargo.toml`](Cargo.toml)

I separated the packages into library and binary using
the [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) feature in package manager

### Cool Trivial Graph Format (TGF) serializer/deserializer

I wrote really cool parser that can handle many errors and nicely print in for the user

`test.tgf`
```
1 First node
2 Second node
2 Check error handler <--- here is duplicated vertex, line 3
#
1 2 Edge between the two
```

```bash
cargo run -- test.tgf --algorithm bfs --start-vertex "First node"
```

My library can handle error and show problems user-friendly

```
Error: graph parse error: vertex with index 2 already defined, check line 3
```

For more details see [`simple-graph/src/error.rs`](simple-graph/src/error.rs)

### Used technology stack

- for [app](app) binary file
    - [dtolnay/thiserror](https://github.com/dtolnay/thiserror) - used to handle complex error without using `Result<T, Box<dyn Error>>`
    - [google/argh](https://github.com/google/argh) - library to process CLI arguments, it's also has small size after compilation
    - [mackwic/colored](https://github.com/mackwic/colored) - BTW, colored output is also used in the binary file
    - [`simple-graph = { path = "../simple-graph" }`](simple-graph)
- for [simple-graph](simple-graph) library
    - [dtolnay/thiserror](https://github.com/dtolnay/thiserror) - see "Cool Trivial Graph Format (TGF) serializer/deserializer" section about the parser
    - [contain-rs/linked-hash-map](https://github.com/contain-rs/linked-hash-map) - mainly used to store vertices in the same order in that it was inserted
    - [alexheretic/linked-hash-set](https://github.com/alexheretic/linked-hash-set) - to store edges in the inserted order
