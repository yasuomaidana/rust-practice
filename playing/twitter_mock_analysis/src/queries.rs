use std::fs;
use neo4rs::{query, Row};
use neo4rs::ConfigBuilder;
use neo4rs::Error;
use neo4rs::Graph;
use neo4rs::Keys;
use neo4rs::Labels;
use neo4rs::Node;

pub(crate) struct FakeTwitterDatabase {
    graph: Graph,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct User {
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

    pub async fn get_user(&self, username: &str) -> Option<String> {
        let get_user_query = fs::read_to_string("./neo4j_definition/get_user.cypher").unwrap();
        let get_user_query = query(get_user_query.as_str());
        let mut result = self.graph.execute(get_user_query.param("username", username)).await.unwrap();
        let user_stored = result.next().await;
        get_user(user_stored, "username")
    }
    pub async fn check_users(&self) -> u64 {
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

    pub async fn create_user(&self, user: String) -> String {
        let create_user_query = fs::read_to_string("./neo4j_definition/create_user.cypher").unwrap();
        let create_user_query = query(create_user_query.as_str());
        let mut result = self.graph.execute(create_user_query.param("username", user.to_string())).await.unwrap();

        let row = result.next().await;

        match get_user(row, "username") {
            Some(username) if username == "" => user,
            Some(username) => username,
            None => "".to_string()
        }
    }
}

fn get_user(result: Result<Option<Row>, Error>, key:&str) -> Option<String> {
    match result {
        Ok(user) => {
            match user {
                Some(user) => {
                    let raw_user:Node = user.get("user").unwrap();
                    raw_user.get(key).ok()
                },
                None => None
            }
        }
        Err(Error::UnexpectedMessage(e)) => {
            if e.contains("Neo.ClientError.Schema.ConstraintValidationFailed") {
                Some("".to_string())
            } else {
                panic!("Unexpected message: {:?}", e);
            }
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }
}