use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];
        let n = candies.len() - 1;
        if n == 0 {
            return 1;
        }
        let mut rs = ratings
            .iter()
            .enumerate()
            .map(|x| (x.0, *x.1))
            .collect::<Vec<_>>();
        rs.sort_unstable_by_key(|x| x.1);
        for (ind, i) in rs.into_iter() {
            if ind == 0 {
                let next = ratings[ind + 1];
                if i > next {
                    candies[ind] = max(candies[ind + 1] + 1, candies[ind]);
                }
            } else if ind == n {
                let prev = ratings[ind - 1];
                if i > prev {
                    candies[ind] = max(candies[ind - 1] + 1, candies[ind]);
                }
            } else {
                let prev = ratings[ind - 1];
                let next = ratings[ind + 1];
                if i > prev {
                    candies[ind] = max(candies[ind - 1] + 1, candies[ind]);
                }
                if i > next {
                    candies[ind] = max(candies[ind + 1] + 1, candies[ind]);
                }
            }
        }
        candies.iter().sum()
    }
}

fn main() {
    println!("Hello, world!");
}
