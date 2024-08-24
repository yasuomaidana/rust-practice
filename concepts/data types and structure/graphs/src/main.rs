use std::collections::BinaryHeap;
use petgraph::Direction;
use petgraph::graph::UnGraph;
use crate::graphs::add_edge;
use crate::models::{Fighter, FighterStats};

mod models;
mod graphs;

fn main() {
    let mut graph = UnGraph::new_undirected();
    let nodes = vec![
        Fighter::new("Ryu".to_string()),
        Fighter::new("Ken".to_string()),
        Fighter::new("Chun-Li".to_string()),
        Fighter::new("Guile".to_string()),
        Fighter::new("Zangief".to_string()),
    ];

    let fighter_nodes = nodes.iter()
        .map(|node| graph.add_node(node))
        .collect::<Vec<_>>();

    add_edge(&mut graph, &fighter_nodes, 0, 1, 1.0);
    add_edge(&mut graph, &fighter_nodes, 1, 3, 1.0);
    add_edge(&mut graph, &fighter_nodes, 3, 0, 1.0);
    add_edge(&mut graph, &fighter_nodes, 3, 2, 1.0);
    add_edge(&mut graph, &fighter_nodes, 3, 4, 1.0);
    add_edge(&mut graph, &fighter_nodes, 0, 4, 1.0);
    add_edge(&mut graph, &fighter_nodes, 2, 4, 1.0);

    let mut ordered_set = BinaryHeap::new();
    for (i, node) in graph.node_indices().enumerate() {
        let name = &graph[node].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("{}: {} - degree: {}, closeness: {}", i, name, degree, closeness);
        ordered_set.push(FighterStats {
            name: name.clone(),
            degree
        });
    }

    println!("\nFighters stats:");
    let max_fighter = ordered_set.iter().max().unwrap().degree;
    let min_fighter = ordered_set.iter().min().unwrap().degree;

    while let Some(fighter) = ordered_set.pop() {
        match fighter.degree {
            fights if fights == max_fighter => println!("{} has the maximum number of combats with {} of fights", fighter.name, fighter.degree+1.0),
            fights if fights == min_fighter => println!("{} has the minimum number of combats with {} of fights", fighter.name, fighter.degree+1.0),
            _ => println!("{} has {} of fights", fighter.name, fighter.degree+1.0)
        }
    }
}
