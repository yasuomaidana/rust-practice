mod queries;

#[tokio::main]
async fn main() {
    let db = queries::FakeTwitterDatabase::new(
        "127.0.0.1:7687",
        "neo4j",
        "test",
        "neo4j"
    ).await;

    let user_stored = db.get_user("johndoe").await;
    match user_stored {
        Some(user) => {
            println!("User found: {:?}", user);
        }
        None => {
            println!("User not found");
        }
    }


}
