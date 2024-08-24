use petgraph::graph::{NodeIndex, UnGraph};
use crate::models::Fighter;

pub(crate) fn add_edge(graph: &mut UnGraph<&Fighter,f32>,
                       nodes: &[NodeIndex],
                       a:usize, b:usize,
                       weight: f32){
    graph.add_edge(nodes[a], nodes[b], weight);
}