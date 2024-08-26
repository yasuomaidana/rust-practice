use std::collections::HashSet;
use std::fs;
use neo4rs::{query, Row};
use neo4rs::ConfigBuilder;
use neo4rs::Error;
use neo4rs::Graph;
use neo4rs::Node;
use crate::user_loader::load_users;

pub(crate) struct FakeTwitterDatabase {
    graph: Graph,
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

    pub async fn get_users(&self) -> HashSet<String> {
        let get_users_query = fs::read_to_string("./neo4j_definition/get_users.cypher").unwrap();
        let get_users_query = query(get_users_query.as_str()).param("limit", 20).param("offset", 0);
        let mut result = self.graph.execute(get_users_query).await.unwrap();
        let mut users = HashSet::new();

        while let Ok(Some(user)) = result.next().await {
            let raw_user: Node = user.get("u").unwrap();
            let username = raw_user.get("username").unwrap();
            users.insert(username);
        }
        users
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

    pub async fn mentions(&self, user1: &str, user2: &str) {
        let mentions_query = fs::read_to_string("./neo4j_definition/mentions.cypher").unwrap();
        let mentions_query = query(mentions_query.as_str());
        let mut result = self.graph
            .execute(
                mentions_query.param("username_1", user1)
                    .param("username_2", user2)).await
            .unwrap();
        let _ = result.next().await;
    }

    pub async fn get_total_mentions(&self) -> u64 {
        let total_mentions_query = fs::read_to_string("./neo4j_definition/total_mentions.cypher").unwrap();
        let total_mentions_query = query(total_mentions_query.as_str());
        let mut result = self.graph.execute(total_mentions_query).await.unwrap();
        let result = result.next().await;
        get_count(result, "count")
    }

    pub async fn clear_users(&self) {
        let clear_users_query = fs::read_to_string("./neo4j_definition/clear_users.cypher").unwrap();
        let queries = clear_users_query.split(";");
        for single_query in queries {
            let clear_users_query = query(single_query);
            let mut result = self.graph.execute(clear_users_query).await.unwrap();
            let _ = result.next().await;
        }
    }

    pub async fn mock_data(&self) {
        let total_users = self.check_users().await;
        let total_mentions = self.get_total_mentions().await;
        if total_users != 13 || total_mentions != 139 {
            self.clear_users().await;
            println!("Loading users");
            load_users(self).await;
        }
    }
}


fn get_user(result: Result<Option<Row>, Error>, key: &str) -> Option<String> {
    match result {
        Ok(user) => {
            match user {
                Some(user) => {
                    let raw_user: Node = user.get("user").unwrap();
                    raw_user.get(key).ok()
                }
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

fn get_count(result: Result<Option<Row>, Error>, key: &str) -> u64 {
    match result {
        Ok(result) => {
            match result {
                Some(result) => {
                    result.get::<u64>(key).unwrap()
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