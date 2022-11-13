struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().filter(|x| x.len() > 0).rev().collect::<Vec<_>>().join(" ")
    }
}

fn main() {
    println!("Hello, world!");
}
