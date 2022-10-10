use std::cmp::Ordering::{Equal, Greater, Less};

struct Solution {}

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let num_sum = nums.iter().sum::<i32>();
        if num_sum % k != 0 {
            return false;
        }
        let group_sum = num_sum / k;

        if nums.iter().any(|&x| x > group_sum) {
            return false;
        }

        let mut prefixSum = vec![0; nums.len()];

        let mut cs = 0;
        let mut j = nums.len() - 1;
        for &i in nums.iter() {
            cs += i;
            prefixSum[j] = cs;
            j -= 1;
        }

        let mut counts = nums.iter().map(|_| false).collect::<Vec<bool>>();
        let mut nums = nums.clone();
        nums.sort();
        // println!("building the sets");
        return Solution::build_sets(
            &prefixSum,
            &(nums.into_iter().rev().collect::<Vec<i32>>()),
            &mut counts,
            group_sum,
            group_sum,
            k - 1,
        );
    }

    pub fn build_sets(
        prefixSum: &Vec<i32>,
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        currentSum: i32,
        targetSum: i32,
        remainingSets: i32,
    ) -> bool {
        // println!("building sets {}/{}", currentSum, remainingSets);
        // println!("{:?}",nums);
        // println!("{:?}",used);
        if currentSum == 0 {
            if remainingSets == 0 {
                return true;
            }
            return Solution::build_sets(prefixSum, nums, used, targetSum, targetSum, remainingSets - 1);
        }

        let mut prev = i32::MAX;

        for i in 0..nums.len() {
            if !used[i] {
                if prefixSum[i] < currentSum {
                    break;
                }
                if prev == nums[i] {
                    continue;
                }
                match (currentSum - nums[i]).cmp(&0) {
                    Less => {
                        continue;
                    }
                    Equal | Greater => {
                        used[i] = true;
                        if Solution::build_sets(
                            prefixSum,
                            nums,
                            used,
                            currentSum - nums[i],
                            targetSum,
                            remainingSets,
                        ) {
                            return true;
                        }
                        used[i] = false;
                    }
                }
                prev = nums[i];
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
