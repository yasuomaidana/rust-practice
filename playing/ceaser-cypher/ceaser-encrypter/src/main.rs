use clap::Parser;

#[derive(Parser, Debug)]
struct Args{
    #[clap(default_value = "")]
    text: String,
    #[clap(short, long, default_value = "3")]
    shift: i32,
}
fn main() {
    let args = Args::parse();
    let text_to_encrypt = args.text;
    let shift = args.shift;

    let cipher = ceaser_cypher_lib::CaesarCipher::new(shift);
    let encrypted_text = cipher.encrypt(&text_to_encrypt);
    println!("Encrypted text: \n{}", encrypted_text);
    let decrypted_text = cipher.decrypt(&encrypted_text);
    println!("Decrypted text: \n{}", decrypted_text);
}
