pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_length = 0;
    let mut char_index = [0; 128]; // ASCII characters
    let mut start = 0;
    for (i, c) in s.chars().enumerate() {
        start = start.max(char_index[c as usize]);
        char_index[c as usize] = i + 1; // Update the last seen index of the character
        max_length = max_length.max(i - start + 1);
    }
    max_length as i32
}
fn main() {
    let s = "abba".to_string();
    let result = length_of_longest_substring(s);
    println!("{}", result);
}
