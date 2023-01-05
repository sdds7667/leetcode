use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut binary_heap = BinaryHeap::from(piles);
        for i in 0..k {
            let y = binary_heap.pop().unwrap();
            binary_heap.push(y-y/2);
        }
        binary_heap.iter().sum()
    }
}


fn main() {
    println!("Hello, world!");
}
