struct Solution {}
use std::cmp::{max, Ordering::Equal};

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if !nums.iter().any(|x| (*x) > 0) {
            return String::from("0");
        }
        let mut nums: Vec<Vec<u8>> = nums
            .iter()
            .map(|x| x.to_string().as_bytes().to_vec())
            .collect();
        nums.sort_unstable_by(|a, b| {
            let al = a.len();
            let bl = b.len();
            let l = max(al, bl);
            for i in 0..l*2 {
                let ac = a[i % al];
                let bc = b[i % bl];
                if ac != bc {
                    return ac.cmp(&bc);
                }
            }

            return Equal;
        });
        nums.reverse();
        let mut res = String::from("");
        for i in nums.iter() {
            for j in i.iter() {
                res.push((*j) as char);
            }
        }
        return res;
    }
}

fn main() {
    println!("Hello, world!");
}
