struct Solution {}

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        // triangle rules: 
        // a + b > c
        let mut heap = std::collections::BinaryHeap::from(nums);
        let mut c = heap.pop().unwrap();
        let mut b = heap.pop().unwrap();
        while let Some(a) = heap.pop() {
            if c < b + a {
                return a + b + c;
            }
            c = b;
            b = a;
        }
        return 0;
    }
}
fn main() {
    println!("Hello, world!");
}
