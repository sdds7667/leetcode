use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let mut maxDiff = i32::MAX;
        let mut bestSum = i32::MIN;
        let nl = nums.len();
        for i in 0..(nl - 2) {
            let mut l = i + 1;
            let mut r = nl - 1;

            while l < r {
                let cs = nums[i] + nums[l] + nums[r];
                let df = target - cs;
                if df == 0 {
                    return target;
                }
                if df.abs() < maxDiff {
                    maxDiff = df.abs();
                    bestSum = cs;
                }
                if cs > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        bestSum
    }
}

fn main() {
    println!("Hello, world!");
    let v = vec![4, 0, 5, -5, 3, 3, 0, -4, -5];
    let target = -2;
    Solution::three_sum_closest(v, target);
}
