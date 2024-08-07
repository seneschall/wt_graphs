//! # Introduction to the WT/-graphs-library
//! This is a powerful tool designed to create and handle various types of graphs. \
//! You can also create normal graphs or create and query so called wt-graphs. \
//! You can call the vertices of your graph by either vertex-indice (usize datatype) or vertex-labels (generic datatype - you choose!).
//! However, you must choose one system per graph or WT graph, as mixing both is not supported.
//! This crate is divided into two main modules: WT and graphs. Here's an overview of the key functionalities:

//! ### 1. Creating and Managing Graphs
//! Using the Graph module, users can create and manipulate eight different types of graphs, depending on whether its edges have directions, and/or weights, and whether the user wants to address the vertices in the graph by name or by index.
//! Graphs can be initialized empty and filled later, or they can be read from a file with a specific format. Users can make changes to these graphs as needed.
//! possible todo()!? note which functions are offered here?

//! ### 2. Working with Wavelet-Tree Graphs
//! The WT module allows users to create four types of Wavelet-Tree graphs, which are graphs offering fast operations using the QWT-library-crate.
//! Wavelet-Tree graphs can be created empty, from an existing graph, or using a bitmap and sequence.
//! Changes to WT graphs are cached and applied only after a commit, which reinitializes the graph.
//!
//! The module provides functions to:
//! - Retrieve information about the initialized graph
//! - perform fast operations on the initialized graph, such as outgoing edges, incoming edges, select, access, BFS, DFS, Shortest Path (fast)
//! - perform the same operations on an "updated" graph that includs the changes you have made. (kinda slow)
//! - perform and query changes on the initialized graph (fast)

//! ### 3. Reading Graph Information from Files
//! The library also offers a module called "from_file" which offers 16 different constructor-functions, one for each of the distinct wt-/graph-types that we offer.
//! These require a specifially formatted file containing the information about the graph. please read the module description for the syntax of the file. \

//! With these functionalities, the wt_graphs library provides a comprehensive and efficient solution for working with both traditional and Wavelet-Tree graphs.

pub mod graph;
pub mod prelude;
pub mod traits;
pub mod wt;

// Enum(s) used by all structures and thus publicly available
#[derive(PartialEq, Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Edit<T> {
    Add(T),
    Delete(T),
}
