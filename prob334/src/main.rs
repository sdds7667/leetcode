struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut c1 = i32::MAX;
        let mut c2 = i32::MAX;
        for i in nums {
            if (i <= c1) {
                c1 = i;
            } else if (i <= c2) {
                c2 = i;
            } else {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    println!("Hello, world!");
}
