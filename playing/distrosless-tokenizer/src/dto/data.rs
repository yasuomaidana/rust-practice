use rocket::serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Text{
    pub text: String
}

#[derive(Deserialize, Serialize)]
pub struct TokenizedText {
    pub tokens: Vec<String>,
}
