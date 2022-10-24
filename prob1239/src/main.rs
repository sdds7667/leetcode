use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut sets = Vec::new();
        for i in arr {
            let mut s = 0;
            let sl = i.len();
            let mut cd = false;
            for j in i.as_bytes() {
                let c = 1 << (*j - 'a' as u8);
                if s & c == c {
                    cd = true;
                    break;
                } else {
                    s |= c;
                }
            }
            if !cd {
                sets.push((sl, s));
            }
        }
        Self::r(&sets, 0, 0, 0) as i32
    }

    pub fn r(arr: &Vec<(usize, i32)>, s: i32, l: usize, i: usize) -> usize {
        let mut m = l;
        for j in i..arr.len() {
            if arr[j].1 & s == 0 {
                m = max(m, Self::r(arr, s | arr[j].1, l + arr[j].0, j+1));
            }
        }
        m
    }
}

fn main() {
    println!("Hello, world!");
}
