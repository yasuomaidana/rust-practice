use serde::Deserialize;

#[derive(Deserialize)]
pub struct Text{
    pub text: String
}

#[derive(Deserialize)]
struct TokenizedText {
    tokens: Vec<String>,
}
