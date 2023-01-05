use std::collections::HashMap;
use std::ptr::hash;

struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut hashmap = HashMap::new();
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let diff = nums[j] as i64 - nums[i] as i64;
                if diff > i32::MAX as i64 || diff < i32::MIN as i64 {
                    continue;
                }
                count += Solution::count_subsequences(&mut hashmap, &nums, j + 1, nums[j] - nums[i], nums[j]);
            }
        }

        count
    }
    pub fn count_subsequences(hashmap: &mut HashMap<(usize, i32), i32>, nums: &Vec<i32>, i: usize, diff: i32, current: i32) -> i32 {
        if hashmap.contains_key(&(i, diff)) {
            return *hashmap.get(&(i, diff)).unwrap();
        }
        let mut res = 0;
        for k in i..nums.len() {
            if current as i64 == nums[k] as i64 - diff as i64 {
                res += 1;
                res += Solution::count_subsequences(hashmap, nums, k + 1, diff, nums[k]);
            }
        }
        hashmap.insert((i, diff), res);
        res
    }
}


fn main() {
    println!("{}", Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]));
    println!("{}", Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]));
    println!("{}", Solution::number_of_arithmetic_slices(vec![0, 2000000000, -294967296]))
}
