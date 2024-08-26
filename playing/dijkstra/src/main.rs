use petgraph::graph::Graph;
use petgraph::algo::dijkstra;
fn main() {
    let mut graph = Graph::new_undirected();
    let mexico_city = graph.add_node("Mexico City");
    let new_york = graph.add_node("New York");
    let los_angeles = graph.add_node("Los Angeles");
    let chicago = graph.add_node("Chicago");
    let houston = graph.add_node("Houston");

    graph.extend_with_edges(&[
        (mexico_city, los_angeles, 1500),
        (mexico_city, chicago, 1700),
        (mexico_city, houston, 1000),
        (new_york, chicago, 800),
        (new_york, houston, 1800),
        (los_angeles, chicago, 2000),
        (los_angeles, houston, 1500),
        (chicago, houston, 1000),
        (houston, new_york, 1800),
    ]);

    let node_map = dijkstra(&graph, mexico_city, Some(new_york),
                            |e| *e.weight());

    if let Some(distance) = node_map.get(&new_york) {
        println!("Distance from Mexico City to New York: {}", distance);
    } else {
        println!("No path found from Mexico City to New York");
    }
}
