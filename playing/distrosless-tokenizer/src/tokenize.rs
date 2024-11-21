use std::thread;
use tokenizers::Tokenizer;

pub async fn tokenize_text(pretrained_model: String, text: String) -> Vec<String> {
    // create a thread to load the tokenizer because this is a blocking call that makes actix panic
    let handle = thread::spawn(move || {
        // create the tokenizer
        return Tokenizer::from_pretrained(pretrained_model, None);
    });

    // shrey update this
    // updated
    let tokenizer = handle.join().expect("Failed to join thread");

    // encode the text using the tokenizer
    let encoded = tokenizer
        .expect("could not create the tokenizer")
        .encode(text.clone(), false)
        .expect("could not read the text");

    // get the tokens from the encoding by unwrapping the result
    let tokens = encoded.get_tokens();
    let tokenized_values = Vec::from(tokens);

    tokenized_values
}