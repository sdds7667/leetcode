use std::cmp::min;

struct Solution{}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if nums[m] > nums[m+1] {
                return nums[m+1];
            }
            if nums[m] > nums[0] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        return nums[r];
    }

    fn find_min_in_interval(nums: &Vec<i32>, l: usize, r: usize) -> i32 {
        if l == r {
            nums[l]
        } else if nums[l] < nums[r] {
            nums[l]
        } else {
            let m = (l+r)/2;
            min(Solution::find_min_in_interval(nums, l, m), Solution::find_min_in_interval(nums, m+1, r))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
