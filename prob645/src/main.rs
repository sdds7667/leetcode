use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        // sum? 1 2 2 4
        // 1 2 3 4

        // 1 + 2 + 2 + 4 = 9
        // 1 + 2 + 3 + 4 = 10

        // 1 + 1 + 2 + 4 = 8
        // 10

        // 1 is the duplicate, 1 + diff is the other number

        // solution O(n) = allocate array of size O(n); Count
        // at the end, return vector containing the
        let n = nums.len();
        let mut res = vec![0; n];
        let mut s = 0;
        let mut d = 0;
        for i in nums {
            res[(i - 1) as usize] += 1;
            s += i;
            if res[(i - 1) as usize] == 2 {
                d = i;
            }
        }

        vec![d, d - s + ((n * (n+1))/2) as i32]
        
    }
}
fn main() {
    println!("Hello, world!");
}
