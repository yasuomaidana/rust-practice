use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};
use scraper::Selector;
use crate::model::Song;

pub async fn load_songs() -> Vec<Song> {
    let client = Client::new();

    // Set the user agent header to mimic a browser request
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"));

    let url = "https://www.chosic.com/free-music/creative-commons/";
    let response = client.get(url).headers(headers).send().await.expect("Failed to send request");
    let scraper = scraper::Html::parse_document(&response.text().await.expect("Failed to parse HTML"));

    scraper
        .select(&Selector::parse("div.track-info").unwrap())
        .map(|song| {
            let title = song.select(&Selector::parse("div.trackF-title-inside").unwrap()).next().unwrap().text().collect::<String>();
            let title = title.trim().to_string();
            let artist = song.select(&Selector::parse("div.artist-track > a").unwrap()).next().unwrap().text().collect::<String>();
            let artist = artist.trim().to_string();
            Song { title, artist }
        }).collect::<Vec<Song>>()
}