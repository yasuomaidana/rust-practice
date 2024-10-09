use std::time;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;

struct ProcessedPage{
    title: String,
    data: String,
}

const PAGES: [&str; 6] = [
    "One Piece",
    "Gene Effect",
    "Altmark incident",
    "The Last of Us",
    "Linkin Park",
    "Metal Gear Solid"
];

fn process_page(page: &Page<Client>) -> ProcessedPage {
    ProcessedPage {
        title: page.get_title().unwrap(),
        data: page.get_content().unwrap(),
    }
}

fn main() {
    let start = time::Instant::now();
    let wikipedia = Wikipedia::<Client>::default();
    let pages:Vec<ProcessedPage> = PAGES
        .par_iter()
        .map(|&title| wikipedia.page_from_title(title.to_string()))
        .map(|page| process_page(&page))
        .collect();

    pages.iter().for_each(|page| {
        let start = time::Instant::now();
        println!("Title: {}", page.title);
        let first_sentence = page.data.split('.').next().unwrap();
        println!("First sentence: {}", first_sentence);
        let words = page.data.split_whitespace().count();
        println!("Word count: {}", words);
        println!("Page time: {:?}\n\n", start.elapsed());
    });
    println!("Total time: {:?}", start.elapsed());
    println!("Average time: {:?}", start.elapsed() / PAGES.len() as u32);
    println!("Total pages: {}", PAGES.len());
    println!("Number of threads: {}", rayon::current_num_threads());
    let total_processed_words = pages.iter().map(|page| page.data.split_whitespace().count()).sum::<usize>();
    println!("Total processed words: {}", total_processed_words);
}
