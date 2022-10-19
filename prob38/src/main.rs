struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }

        let prev = Self::count_and_say(n - 1);
        println!("n={}; prev={}", n, prev);

        let mut last = prev.chars().nth(0).unwrap();
        let mut count = 1;

        let mut res = String::from("");

        for i in prev.chars().skip(1) {
            if last == i {
                count += 1;
            } else {
                res.push_str(&format!("{}{}", last, count));
                last = i;
            }
        }
        res.push_str(&format!("{}{}", last, count));
        res
    }
}

fn main() {
    println!("{}", Solution::count_and_say(4));
}
