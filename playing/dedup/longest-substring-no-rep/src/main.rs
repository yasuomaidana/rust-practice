use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    let mut tail = 0;
    let mut head = 0;
    let mut using_chars: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    while head < chars.len() {
        if !using_chars.contains(&chars[head]) {
            using_chars.insert(chars[head]);
            max = max.max((head - tail + 1) as i32);
            head += 1;
        } else {
            using_chars.remove(&chars[tail]);
            tail += 1;
        }
    }
    max
}
fn main() {
    println!("Hello, world!");
}
