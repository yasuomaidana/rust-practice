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
    pub async fn check_users(&self)-> u64{
        let check_users_query = fs::read_to_string("./neo4j_definition/check_users.cypher").unwrap();
        let check_users_query = query(check_users_query.as_str());
        let mut result = self.graph.execute(check_users_query).await.unwrap();
        let result = result.next().await;
        match result {
            Ok(result) => {
                match result {
                    Some(result) => {
                        result.get::<u64>("count").unwrap()
                    }
                    None => {
                        0
                    }
                }
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
}