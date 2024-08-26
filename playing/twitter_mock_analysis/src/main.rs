use crate::user_loader::load_users;

mod queries;
mod user_loader;

#[tokio::main]
async fn main() {
    let db = queries::FakeTwitterDatabase::new(
        "127.0.0.1:7687",
        "neo4j",
        "test",
        "neo4j"
    ).await;

    let total_users = db.check_users().await;
    let total_mentions = db.get_total_mentions().await;
    if total_users != 13 && total_mentions != 139 {
        db.clear_users().await;
        println!("Loading users");
        load_users(&db).await;
    }
    println!("Users count: {:?}", db.check_users().await);
    println!("Total comments: {:?}", db.get_total_mentions().await);

}
