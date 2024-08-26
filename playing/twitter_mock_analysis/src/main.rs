mod queries;

use std::fs;
use neo4rs::{query, ConfigBuilder, Graph};

#[tokio::main]
async fn main() {
    let config = ConfigBuilder::default()
        .uri("127.0.0.1:7687")
        .user("neo4j")
        .password("test")
        .db("neo4j")
        .fetch_size(500)
        .max_connections(10)
        .build()
        .unwrap();
    let graph = Graph::connect(config).await.unwrap();
    let get_user_query = fs::read_to_string("./neo4j_definition/get_user.cypher").unwrap();
    let get_user_query = query(get_user_query.as_str());
    let mut result = graph.execute(get_user_query.param("username", "rightnpr")).await.unwrap();
    let user_stored = result.next().await;
    match user_stored {
        Ok(user) => {
            match user {
                Some(user) => {
                    println!("User: {:?}", user);
                }
                None => {
                    println!("User not found");
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }


}
