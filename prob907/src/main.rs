use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut mod_val = 100000007;

        let mut dp = vec![vec![i32::MAX; 2*arr.len()]; 2];
        let mut dpi = 0;

        let mut s = 0;
        for i in 0..arr.len() {
            dp[dpi][i] = arr[i];
            s = (s+dp[dpi][i]) % mod_val;
        }

        for i in 1..arr.len() {
            for j in 0..(arr.len()-i) {
                dp[dpi][j] = min(dp[(dpi+1)%2][j], dp[(dpi+1)%2][j+1]);
                s = (s+dp[dpi][j]) % mod_val;
            }
            dpi = (dpi + 1) % 2;
        }
        s

    }
}

fn main() {
    println!("Hello, world!");
}
