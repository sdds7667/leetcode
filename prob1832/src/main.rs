use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
                        ('a'..='z').into_iter().collect::<HashSet<char>>()
            == sentence.chars().collect::<HashSet<char>>()
    }
}

fn main() {
    Solution::check_if_pangram(String::from("abce"));
}
