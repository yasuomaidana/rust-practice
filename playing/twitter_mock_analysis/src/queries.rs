use std::error::Error;
use std::fs;
use neo4rs::{query, ConfigBuilder, Graph, Keys, Labels, Neo4jError, Node};
use neo4rs::Error::Neo4j;

pub(crate) struct FakeTwitterDatabase{
    graph: Graph
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct  User{
    labels: Labels,
    keys: Keys<Vec<String>>,
    username: String,
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

    pub async fn get_user(&self, username: &str) -> Option<User> {
        let get_user_query = fs::read_to_string("./neo4j_definition/get_user.cypher").unwrap();
        let get_user_query = query(get_user_query.as_str());
        let mut result = self.graph.execute(get_user_query.param("username", username)).await.unwrap();
        let user_stored = result.next().await;
        match user_stored {
            Ok(user) => {
                match user {
                    Some(user) => user.get("username").ok(),
                    None => None
                }
            }
            Err(e) => {
                panic!("Error: {:?}", e);
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

    pub async fn create_user(&self, user:String)-> String{
        let create_user_query = fs::read_to_string("./neo4j_definition/create_user.cypher").unwrap();
        let create_user_query = query(create_user_query.as_str());
        let mut result = self.graph.execute(create_user_query.param("username", user)).await.unwrap();

        let Ok(Some(user)) = result.next().await else {panic!("Error creating user")};

        let node:Node = user.get("user").unwrap();
        node.get::<String>("username").unwrap()
    }
}