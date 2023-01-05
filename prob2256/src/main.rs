struct Solution;

impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
        let sum: i64 = nums.iter() .sum();
        let mut rs = sum;
        let mut ls = 0;
        let mut min_diff = i64::MAX;
        let mut min_diff_index = 0;
        for i in 0..nums.len()-1 {
            ls += nums[i];
            rs -= nums[i];
            let diff = ((ls / ((i+1) as i64)) - (rs / ((nums.len() - i - 1) as i64))).abs();
            if diff < min_diff {
                min_diff = diff;
                min_diff_index = i;
            }
        }
        if sum / (nums.len() as i64) < min_diff {
            return (nums.len()-1) as i32;
        }
        min_diff_index as i32
    }
}


fn main() {
    println!("Hello, world!");
}
