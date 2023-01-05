struct Solution {}

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut stack = Vec::new();
        let mut i = 0;
        let n = nums.len();
        while i < nums.len() {
            let cn = nums[i];
            while let Some(&x) = stack.last() {
                if x > cn && stack.len() + n - i> k {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.len() != k {
                stack.push(cn);
            }
            i += 1;
        }
        stack
    }
}

fn main() {
    println!("{:?}", Solution::most_competitive(vec![2,4,3,3,5,4,9,6], 4));
}
