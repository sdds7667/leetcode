struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable_by(|a,b| b.cmp(a));

        let mut dp = vec![vec![0; satisfaction.len() + 1]; 2];

        // take it: Increase the time by 1, and move to the next dish + self.satisfaction * time.
        // don't take it: Don't increase the time by 1, move to the next dish.

        let mut vi = 0;
        let mut changes;

        for i in 0..satisfaction.len() {
            let s = satisfaction[i];
            changes = false;
            for j in 0..satisfaction.len() {
                let take = s * (j as i32 + 1) + dp[1 - vi][j + 1];
                if dp[1-vi][j] < take{
                    dp[vi][j] = take;
                    changes = true;
                } else {
                    dp[vi][j] = dp[1-vi][j];
                }
            }
            if !changes {
                return dp[vi][0];
            }
            vi = 1-vi;
        }


        return dp[1-vi][0] as i32;
    }
}

fn main() {
    Solution::max_satisfaction(vec![-1,-8,0,5,-7]);
}
