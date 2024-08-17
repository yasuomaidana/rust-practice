use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use scraper::Selector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new HTTP client
    let client = Client::new();

    // Set the user agent header to mimic a browser request
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));

    // Make a GET request to the website
    let response = client.get("https://www.scrapingcourse.com/ecommerce/").headers(headers).send().await?;

    // Create a new Scraper from the HTML response
    let scraper = scraper::Html::parse_document(&response.text().await?);

    let products = scraper.select(&Selector::parse("li.product").unwrap())
        .map(|product| {
            let product_name = product.select(&Selector::parse("h2.product-name").unwrap()).next().unwrap().text().collect::<String>();
            let price = product.select(&Selector::parse("span.price > span.woocommerce-Price-amount > bdi").unwrap()).next().unwrap().text().collect::<String>();
            (product_name, price)
        })
        .collect::<Vec<(String, String)>>();

    for (product_name, price) in products {
        println!("Product Name: {}", product_name);
        println!("Price: {}", price);
    }

    Ok(())
}
