mod queries;
mod user_loader;

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
}
