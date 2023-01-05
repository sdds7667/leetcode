
struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count_map = HashMap::new();
        for i in arr {
            let mut entry = count_map.entry(i).or_insert(0);
            *entry += 1;
        }
        let mut values_set = HashSet::new();
        for i in count_map.values() {
            if !values_set.insert(*i) {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
