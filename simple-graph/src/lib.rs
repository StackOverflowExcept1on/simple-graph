//! Graph library with ability to serialize/deserialize Trivial Graph Format
//!
//! Besides serialize/deserialize library can deal with graph algorithms such
//! as [Depth-first search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search)
//! and [Breadth-first search (BFS)](https://en.wikipedia.org/wiki/Breadth-first_search).
//!
//! If you are looking for example visit [`Graph`]

#![feature(str_split_whitespace_remainder)]

pub use error::*;
pub use graph::*;
pub use tgf::*;

mod error;
mod graph;
mod tgf;
