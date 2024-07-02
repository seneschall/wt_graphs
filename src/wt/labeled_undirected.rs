use crate::traits::{Graph, Labeled, Undirected, Unweighted, WTLabeled, WTUndirected, WT};
use crate::wt::labeled_directed::LabeledWTDigraph;
use std::hash::Hash;

/// An labeled wavelet-tree-graph with undirected edges. (L-wt-ugraph)
/// The L-wt-ugraph holds a L-wt-digraph. All operations on the L-wt-digraph can be performed on the L-wt-ugraph. 
/// The only divergent implementations are regarding the "doubling" of edges due to no directions.
/// Users can perfom fast operations on the original graph and slower operations on the recent state of the graph.
/// Users can integrate the recent state of the graph into the QW-Tree by rebuilding it using the commit_edits-function.
/// See module wt::labeled_directed for the L-wt-digraph struct definition. See more documentation on function-level and in the crate introduction.
/// The greatest possible of number of edges or of vertices is usize, vertex-indices are also usize-data-type. Labels can have any type and are referenced.
pub struct LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    ldg: LabeledWTDigraph<L>,
}

impl<L> LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    pub fn from_digraph() {
        todo!()
    }
    pub fn from() {
        todo!()
    }
}
impl<L> Graph<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    /// this function needs documentation
    fn add_vertex(&mut self, vertex: L) -> usize {
        todo!()
    }

    fn e_count(&self) -> usize {
        todo!()
    }

    fn v_count(&self) -> usize {
        todo!()
    }

    /// this function needs documentation
    fn delete_edge(&mut self, from: L, to: L) {
        todo!()
    }

    fn delete_vertex(&mut self, vertex: L) {
        todo!()
    }

    fn vertex_exists(&self, vertex: L) -> bool {
        todo!()
    }

        /// this function needs documentation
    fn shrink(&mut self) -> std::collections::HashMap<usize, usize> {
        todo!()
    }

    fn edge_exists(&self, from: L, to: L) -> bool {
        todo!()
    }
}
impl<L> Undirected<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges(&self, vertex: L) -> Vec<L> {
        todo!()
    }

    fn delete_edges_from(&mut self, vertex: L) {
        todo!()
    }
}
impl<L> Labeled<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edit_label(&mut self, old_label: L, new_label: L) {
        todo!()
    }

    fn get_label(&self, vertex: usize) -> Option<&L> {
        todo!()
    }

    /// this function needs documentation
    fn get_index(&self, label: L) -> Option<&usize> {
        todo!()
    }
}
impl<L> Unweighted<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
        /// this function needs documentation
    fn add_edge(&mut self, from: L, to: L) {
        todo!()
    }
}
impl<L> WT<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn commit_edits(&mut self) {
        todo!()
    }

    // fn get_uncommitted_edits(&self) -> Option<std::collections::HashMap<usize, L>> {
    //     todo!()
    // }

    fn discard_edits(&mut self) {
        todo!()
    }

    fn vertex_exists_updated(&self, vertex: L) -> bool {
        todo!()
    }

    fn edge_exists_updated(&self, from: L, to: L) -> bool {
        todo!()
    }
    fn v_count_updated(&self) -> usize { todo!() }
}
impl<L> WTUndirected<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn edges_updated(&self, vertex: usize) -> Vec<L> {
        todo!()
    }
}
impl<L> WTLabeled<L> for LabeledWTUGraph<L>
where
    L: Hash + Eq + Clone,
{
    fn get_label_updated(&self, index: usize) -> Option<&L> {
        todo!()
    }

    fn get_index_updated(&self, label: L) -> Option<&usize> {
        todo!()
    }
}
