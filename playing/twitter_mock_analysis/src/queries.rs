use std::fs;
use neo4rs::{query, ConfigBuilder, Graph};

pub(crate) struct FakeTwitterDatabase{
    graph: Graph
}

impl FakeTwitterDatabase {
    pub async fn new(uri: &str, user: &str, password: &str, db: &str) -> Self {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .db(db)
            .fetch_size(500)
            .max_connections(10)
            .build()
            .unwrap();
        FakeTwitterDatabase {
            graph: Graph::connect(config).await.unwrap()
        }
    }

    pub async fn get_user(&self, username: &str) -> Option<neo4rs::Row> {
        let get_user_query = fs::read_to_string("./neo4j_definition/get_user.cypher").unwrap();
        let get_user_query = query(get_user_query.as_str());
        let mut result = self.graph.execute(get_user_query.param("username", username)).await.unwrap();
        let user_stored = result.next().await;
        match user_stored {
            Ok(user) => {
                match user {
                    Some(user) => {
                        Some(user)
                    }
                    None => {
                        None
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }
}