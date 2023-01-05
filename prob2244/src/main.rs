use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut count_map = HashMap::new();
        for &i in tasks.iter() {
            let entry = count_map.entry(i).or_insert(0);
            *entry += 1;
        }
        let mut rounds = 0;
        for &i in count_map.values() {
            if i == 1 {
                return -1;
            }
            if i % 3 == 0 {
                rounds += i / 3;
            } else {
                rounds += i / 3 + 1;
            }
        }
        rounds
    }
}

fn main() {
    println!("Hello, world!");
}
