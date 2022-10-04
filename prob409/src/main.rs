
struct Solution{}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut hm = std::collections::HashMap::new();

        for i in s.chars() {
            match hm.get(&i) {
                Some(&x) => hm.insert(i, x + 1),
                None => hm.insert(i, 1),
            };
        }

        let mut odd = 0;
        let mut pairs = 0;
        for &i in hm.values() {
            pairs += i / 2;
            if i % 2 == 1 {
                odd = 1;
            }
        }
        odd + pairs * 2
    }
}

fn main() {
    println!("Hello, world!");
}
