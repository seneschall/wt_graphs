use std::collections::HashMap;

use vers_vecs::RsVec;

use crate::graph::weighted_directed::WeightedDigraph;
use crate::traits::{Directed, Graph, Unlabeled, Unweighted, WTDirected, WTWeighted, Weighted, WT};
use crate::wt::directed::WTDigraph;
use crate::Edit;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

// A structure holding an immutable Wavelet-Tree-Representation of an indexed graph with directed edges, where each edge represents a weight, plus information on manual changes.
// The greatest possible of number of edges or of vertices is usize vertices, vertex-indices are also usize-data-type. Weights can have any type.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeightedWTDigraph<W> {
    dg: WTDigraph,
    weights_uncommitted: HashMap<(usize, usize), Edit<W>>,
    weights: HashMap<(usize, usize), W>,
}

impl<W> WeightedWTDigraph<W> {
    pub fn from_weighted_digraph(wdg: WeightedDigraph<W>) -> Self {
        return WeightedWTDigraph {
            dg: WTDigraph::from_digraph(wdg.dg),
            weights_uncommitted: HashMap::new(),
            weights: wdg.weights,
        };
    }
    pub fn from(
        sequence: Vec<usize>,
        starting_indices: RsVec,
        weights: HashMap<(usize, usize), W>,
    ) -> Self {
        return WeightedWTDigraph {
            dg: WTDigraph::from(sequence, starting_indices),
            weights_uncommitted: HashMap::new(),
            weights,
        };
    }
}
impl<W> Graph<usize> for WeightedWTDigraph<W>
where
    W: Clone,
{
    fn add_vertex(&mut self, vertex: usize) -> usize {
        self.dg.add_vertex(vertex)
    }

    fn e_count(&self) -> usize {
        self.dg.e_count()
    }

    fn v_count(&self) -> usize {
        self.dg.v_count()
    }

    fn delete_edge(&mut self, from: usize, to: usize) {
        self.dg.delete_edge(from, to);
        let old_weight = self.weight_updated(from, to); // checkme
        self.weights_uncommitted
            .insert((from, to), Edit::Delete(old_weight));
    }

    fn delete_vertex(&mut self, vertex: usize) {
        self.dg.delete_vertex(vertex);
    }

    fn vertex_exists(&self, vertex: usize) -> bool {
        self.dg.vertex_exists(vertex)
    }

    fn edge_exists(&self, from: usize, to: usize) -> bool {
        if from <= to {
            self.dg.edge_exists(from, to)
        } else {
            self.dg.edge_exists(to, from)
        }
    }
}

impl<W> Directed<usize> for WeightedWTDigraph<W>
where
    W: Clone,
{
    fn outgoing_edges(&self, vertex: usize) -> Vec<usize> {
        self.dg.outgoing_edges(vertex)
    }

    fn incoming_edges(&self, vertex: usize) -> Vec<usize> {
        self.dg.incoming_edges(vertex)
    }

    fn delete_outgoing_edges(&mut self, vertex: usize) {
        // checkme
        if !self.vertex_exists_updated(vertex) {
            panic!("delete_outgoing_edges: Vertex {} doesn't exist.", vertex);
        }

        let outgoing: Vec<usize> = self.outgoing_edges_updated(vertex);

        for to in outgoing {
            self.delete_edge(vertex, to); // this function call updates e_count_updated
        }
        // need to call self.delete_edge to update weights
        self.dg.has_uncommitted_edits = true;
    }

    fn delete_incoming_edges(&mut self, vertex: usize) {
        // checkme
        if !self.vertex_exists_updated(vertex) {
            panic!("incoming_edges: Vertex {} doesn't exist.", vertex);
        }

        let incoming: Vec<usize> = self.incoming_edges_updated(vertex); // empty list if there are no incoming edges

        for from in incoming {
            self.delete_edge(from, vertex); // this function call updates e_count_updated
        }
        // need to call self.delete_edge to update weights
        self.dg.has_uncommitted_edits = true;
    }
}

impl<W> Unlabeled<usize> for WeightedWTDigraph<W> {
    fn append_vertex(&mut self) -> usize {
        self.dg.append_vertex()
    }

    fn shrink(&mut self) -> Vec<Option<usize>> {
        self.dg.shrink()
    }
}
impl<W> Weighted<usize, W> for WeightedWTDigraph<W>
where
    W: Clone,
{
    fn add_edge(&mut self, from: usize, to: usize, weight: W) {
        self.dg.add_edge(from, to);
        self.weights_uncommitted
            .insert((from, to), Edit::Add(weight));
    }

    fn edit_weight(&mut self, from: usize, to: usize, weight: W) {
        if self.edge_exists_updated(from, to) {
            self.weights_uncommitted
                .insert((from, to), Edit::Add(weight));
        }
    }

    fn weight(&mut self, from: usize, to: usize) -> W {
        if self.edge_exists(from, to) {
            if self.weights.contains_key(&(from, to)) {
                let weight = self.weights.get(&(from, to)).unwrap().clone();
                return weight;
            } else {
                panic!("wdg get_weight : weight is missing")
            }
        }
        panic!("wdg get_weight : edge does not exist");
    }
}

impl<W> WT<usize> for WeightedWTDigraph<W>
where
    W: Clone,
{
    fn commit_edits(&mut self) {
        for ((from, to), change) in self.weights_uncommitted.iter() {
            match change {
                Edit::Add(add_weight) => {
                    if !self.weights.contains_key(&(*from, *to)) {
                        self.weights.insert((*from, *to), add_weight.clone());
                    }
                }
                Edit::Delete(_) => {
                    if self.weights.contains_key(&(*from, *to)) {
                        self.weights.remove(&(*from, *to));
                    }
                }
            }
        }
        self.weights_uncommitted = HashMap::new();
        self.dg.commit_edits();
    }

    fn discard_edits(&mut self) {
        self.dg.discard_edits();
        self.weights_uncommitted = HashMap::new();
    }

    fn vertex_exists_updated(&self, vertex: usize) -> bool {
        self.dg.vertex_exists_updated(vertex)
    }

    fn edge_exists_updated(&self, from: usize, to: usize) -> bool {
        self.dg.edge_exists_updated(from, to)
    }

    fn v_count_updated(&self) -> usize {
        self.dg.v_count_updated()
    }

    fn e_count_updated(&self) -> usize {
        return self.dg.e_count_updated();
    }
}
impl<W> WTDirected<usize> for WeightedWTDigraph<W> {
    fn outgoing_edges_updated(&self, vertex: usize) -> Vec<usize> {
        self.dg.outgoing_edges_updated(vertex)
    }

    fn incoming_edges_updated(&self, vertex: usize) -> Vec<usize> {
        self.dg.incoming_edges_updated(vertex)
    }
}
impl<W> WTWeighted<usize, W> for WeightedWTDigraph<W>
where
    W: Clone,
{
    fn weight_updated(&mut self, from: usize, to: usize) -> W {
        if !self.vertex_exists_updated(from) {
            panic!("wdg get_weight_updated : from Vertex doesn't exist");
        }
        if !self.vertex_exists_updated(to) {
            panic!("wdg get_weight_updated : to Vertex doesn't exist");
        }
        if !self.edge_exists_updated(from, to) {
            panic!("wdg get_weight_updated : edge from {from} to {to} does not exist");
        }
        // check if weight was editted
        if self.weights_uncommitted.contains_key(&(from, to)) {
            match self.weights_uncommitted.get(&(from, to)).unwrap() {
                Edit::Add(weight) => {
                    return weight.clone();
                }
                Edit::Delete(_) => {
                    panic!("wdg get_weight_updated : edge_exists_updated but weight is deleted");
                }
            }
        } else {
            return self.weight(from, to);
        }
    }
}
