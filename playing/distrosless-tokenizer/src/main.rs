
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use distrosless_tokenizer::dto::data::Text;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/tokenizers/<pretrained_model>", data="<text>")]
fn tokenize(pretrained_model: &str, text: Json<Text>) -> String{
    let mut return_text:String = String::new();
    let str1 = format!("Tokenizing text using the model: {}", pretrained_model);
    return_text.push_str(&str1);
    let str2 = format!("\nText: {}", text.text);
    return_text.push_str(&str2);
    return_text
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, tokenize])
}
