struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        Solution::mincost_tickets_iter(&days, &costs)
    }

    pub fn mincost_tickets_iter(days: &Vec<i32>, costs: &Vec<i32>) -> i32 {
        let mut dp = vec![0; days.len() + 1];
        for i in (0..days.len()).rev() {
            let mut min_cost = costs[0] + dp[i + 1];
            let mut j = i + 1;
            while j < days.len() {
                if days[j] > (days[i] + 6) {
                    break;
                } else {
                    j += 1;
                }
            }
            min_cost = min_cost.min(costs[1] + dp[j]);

            while j < days.len() {
                if days[j] > (days[i] + 29) {
                    break;
                } else {
                    j += 1;
                }
            }

            min_cost = min_cost.min(costs[2] + dp[j]);
            dp[i] = min_cost;
        }

        return dp[0];
    }


    pub fn mincost_tickets_rec(days: &Vec<i32>, costs: &Vec<i32>, i: usize, mem: &mut Vec<Option<i32>>) -> i32 {
        if i == days.len() {
            return 0;
        }
        if let Some(x) = mem[i] {
            return x;
        }

        let mut min_cost = costs[0] + Solution::mincost_tickets_rec(days, costs, i + 1, mem);

        let mut j = i + 1;
        while j < days.len() {
            if days[j] > (days[i] + 6) {
                break;
            } else {
                j += 1;
            }
        }
        min_cost = min_cost.min(costs[1] + Solution::mincost_tickets_rec(days, costs, j, mem));

        while j < days.len() {
            if days[j] > (days[i] + 29) {
                break;
            } else {
                j += 1;
            }
        }

        min_cost = min_cost.min(costs[2] + Solution::mincost_tickets_rec(days, costs, j, mem));
        mem[i] = Some(min_cost);
        return min_cost;
    }
}

fn main() {
    println!("Hello, world!");
}
