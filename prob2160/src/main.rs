struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        /*
            Example:
            digits = [i,j,k,l] i <= j <= k <= l
            i * 10 + j * 10 + k + l

            digits = [i,j,k]
            i * 10 + j + k

            digits = [i,j]
            i + j

            digits = [i]
            i
        */

        let mut v = Vec::new();
        let mut num = num;
        while num > 0 { 
            v.push(num % 10);
            num /= 10;
        }

        v.sort_unstable();
        let multipliers = vec![1,1,10,10];
        let mut i = 0;
        let mut s = 0;
        while !v.is_empty() {
            s += multipliers[i] * v.pop().unwrap();
            i += 1;
        }
        s
    }
}

fn main() {
    println!("Hello, world!");
}
