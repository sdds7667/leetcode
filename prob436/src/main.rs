use std::borrow::BorrowMut;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut intervals_clone = intervals.clone();
        for i in 0..intervals_clone.len() {
            intervals_clone[i].push(i as i32);
        }
        intervals_clone.sort_unstable_by_key(|x| x[0]);
        let mut r = HashMap::new();
        for i in 0..intervals_clone.len() {
            let mut right = -1;
            for j in i..intervals_clone.len() {
                if intervals_clone[j][0] >= intervals_clone[i][1] {
                    right = intervals_clone[j][2] as i32;
                    break;
                }
            }
            r.insert(&intervals_clone[i][0], right);
        }

        intervals.into_iter().map(|x| *(r.get(&x[0]).unwrap())).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
