struct Solution {}

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        Self::r(&mut std::collections::HashMap::new(), n)
    }

    fn r(dp: &mut std::collections::HashMap<i32, i32>, n: i32) -> i32 {
        if n == 1 {
            0
        } else if let Some(x) = dp.get(&n) {
            *x
        } else {
            let res = if n % 2 == 0 {
                1 + Self::r(dp, n / 2)
            } else {
                let m = (n - 1).count_ones();
                let p = (n + 1).count_ones();
                if m == p {
                    1 + std::cmp::min(Self::r(dp, n - 1), Self::r(dp, n + 1))
                } else if m < p {
                    1 + Self::r(dp, n - 1)
                } else {
                    1 + Self::r(dp, n + 1)
                }
            };
            dp.insert(n, res);
            res
        }
    }
}

fn main() {
    println!("Hello, world!");
}
