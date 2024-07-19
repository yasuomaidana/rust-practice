use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fact {
    fact: String,
    length: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    const CHECK_URL: &str = "https://catfact.ninja/fact";

    let client = Client::new();
    let response = client.get(CHECK_URL).send().await?;

    if !response.status().is_success() {
    // Handle non-successful HTTP status codes by returning a generic error
    panic!("Panic! HTTP status code: {}", response.status());
}

    let result: Fact = response.json().await?;

    println!("Result: {:?}", result);

    if result.length > 100 {
        panic!("Panic! Fact is too long");
    }

    // Check for non-English characters
   for ch in result.fact.chars() {
       if !ch.is_ascii_alphanumeric() && ![',', '.', '!', '?', '"','”','“',' ','-'].contains(&ch) {
           println!("Non-English character: {}", ch);
           panic!("Panic! Fact contains non-English character");
       }
   }

    println!("Fact: {}", result.fact);
    Ok(())
}