use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use crate::models::{ErrorResponse, Pokemon};

const CACHE_DURATION: Duration = Duration::from_secs(3600); // Cache for 1 hour
static POKEMON_API_URL: &str = "https://pokeapi.co/api/v2/pokemon/";

// Global cache map (using Mutex for thread-safety)
lazy_static::lazy_static! {
    static ref POKEMON_CACHE: Mutex<HashMap<String, (Pokemon, Instant)>> = Mutex::new(HashMap::new());
}

async fn fetch_pokemon(name: &str) -> Result<Pokemon, ErrorResponse> {
    let url = format!("{}/{}", POKEMON_API_URL, name);
    let response = reqwest::get(&url).await.unwrap(); // Use reqwest for the API call

    if response.status().is_success() {
        let pokemon: Pokemon = response.json().await.unwrap();
        Ok(pokemon)
    } else {
        Err(ErrorResponse {
            error: format!("Error fetching data for Pokemon: {}", name),
        })
    }
}

// Wrapper function to interact with the cache
pub(crate) async fn get_cached_pokemon(name: &str) -> Result<Pokemon, ErrorResponse> {
    let mut cache = POKEMON_CACHE.lock().await;

    // Check if the entry exists and is not expired
    if let Some((pokemon, expiry)) = cache.get(name) {
        if expiry > &Instant::now() {
            println!("Pokemon {} found in cache", name);
            return Ok(pokemon.clone());
        } else {
            // Entry is expired, remove it
            cache.remove(name);
        }
    }

    println!("Pokemon {} not in cache, fetching...", name);
    let fetched_pokemon = fetch_pokemon(name).await?;
    cache.insert(name.to_owned(), (fetched_pokemon.clone(), Instant::now() + CACHE_DURATION));
    Ok(fetched_pokemon)
}