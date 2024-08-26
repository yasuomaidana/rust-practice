use std::collections::HashMap;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::DiGraph;

mod queries;
mod user_loader;

///
/// This is a simple example of a social network analysis using the petgraph crate.
/// The FakeTwitterDatabase struct is a mock of a real database connection to a Neo4j database.
/// The user_loader module is a mock of a real user loader.
/// The main function creates a graph of users and their mentions and then uses Kosaraju's algorithm to find communities.
///
/// ![Graph image](graph.png)
///
#[tokio::main]
async fn main() {
    let db = queries::FakeTwitterDatabase::new(
        "127.0.0.1:7687",
        "neo4j",
        "test",
        "neo4j",
    ).await;
    db.mock_data().await;

    println!("Users count: {:?}", db.check_users().await);
    println!("Total comments: {:?}", db.get_total_mentions().await);

    let mut graph = DiGraph::new();
    let mut stored_users = HashMap::new();

    let users = db.get_users().await;
    users.iter().for_each(|user| {
        stored_users.insert(user.clone(), graph.add_node(user.clone()));
    });

    for user in users{
        let mentioned_users = db.mentioned_users(&user).await;
        for (mentioned_user, count) in mentioned_users{
            for _ in 0..count{
                let user_node = stored_users.get(&user).unwrap();
                let mentioned_user_node = stored_users.get(&mentioned_user).unwrap();
                graph.add_edge(*user_node, *mentioned_user_node, "mention");
            }
        }
    }

    // Kosaraju's algorithm
    let mut sccs = kosaraju_scc(&graph);
    sccs.sort_by(|a, b| b.len().cmp(&a.len()));
    for community in sccs{
        println!("Community of {}", community.len());
        let usernames: Vec<String> = community.iter()
            .map(|&node| graph[node].clone()).collect();
        println!("{:?}", usernames);
    }
}
