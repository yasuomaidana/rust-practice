
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use distrosless_tokenizer::dto::data::{Text, TokenizedText};
use distrosless_tokenizer::tokenize::tokenize_text;

#[get("/")]
fn index() -> &'static str {
    "Summarization API"
}

#[post("/tokenizers/<pretrained_model>", data="<text>")]
async fn tokenize(pretrained_model: &str, text: Json<Text>) -> Json<TokenizedText>{

    let tokenized_text = tokenize_text(pretrained_model.to_string(), text.text.clone());
    let tokenized_text = tokenized_text.await;
    Json(TokenizedText{tokens: tokenized_text})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, tokenize])
}
