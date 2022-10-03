use std::cmp::{min, max};

struct Solution {}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut prev = '0';
        let mut prev_cost = 0;
        let mut result = 0;
        for (ind, i) in colors.chars().enumerate() {
            if i == prev {
                result += min(prev_cost, needed_time[ind]);
                prev_cost = max(prev_cost, needed_time[ind]);
            } else {
                prev = i;
                prev_cost = needed_time[ind];
            }
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}
