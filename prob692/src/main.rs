struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut word_count = HashMap::new();

        for i in words {
            if let Some(x) = word_count.get(&i) {
                word_count.insert(i, x+1);
            } else {
                word_count.insert(i, 1);
            }
        }

        let mut tuple_count : Vec<(String, i32)> = word_count.into_iter().collect();
        tuple_count.sort_unstable_by(|x, y| if x.1 == y.1 {x.0.cmp(&y.0)} else {y.1.cmp(&x.1)});
        tuple_count.drain((k as usize)..tuple_count.len());
        tuple_count.into_iter().map(|x| x.0).collect()
    }
}
fn main() {
    println!("Hello, world!");
}
