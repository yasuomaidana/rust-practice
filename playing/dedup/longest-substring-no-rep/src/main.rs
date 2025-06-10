use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut tail = 0;
    let mut head = 0;
    let mut tails = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    while head < chars.len() {
        if tails.contains_key(&chars[head]) {
            tail = tail.max(tails.remove(&chars[head]).or_else(|| Some(0)).unwrap() + 1);
        } else {
            tails.insert(chars[head], head);
            max = max.max(head - tail + 1);
            head += 1;
        }
    }
    max as i32
}
fn main() {
    let s = "abba".to_string();
    let result = length_of_longest_substring(s);
    println!("{}", result);
}
