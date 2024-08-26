use neo4rs::{ConfigBuilder, Graph};

pub(crate) struct FakeTwitterDatabase{
    db: String,
    graph: Graph
}

impl FakeTwitterDatabase {
    pub async fn new(uri: &str, user: &str, password: &str, db: &str) -> Self {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .db("neo4j")
            .fetch_size(500)
            .max_connections(10)
            .build()
            .unwrap();
        FakeTwitterDatabase {
            db: db.to_string(),
            graph: Graph::connect(config).await.unwrap()
        }
    }
}