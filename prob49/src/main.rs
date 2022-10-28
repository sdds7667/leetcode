
struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = std::collections::HashMap::new();
        for i in strs.iter() {
            let mut b: Vec<_> = i.chars().collect();
            b.sort_unstable();
            hm.entry(String::from_iter(b)).or_insert(Vec::new()).push(i.clone());
        }
        hm.into_iter().map(|x| x.1).collect()
    }
}

fn main() {
    println!("{:?}",Solution::group_anagrams(vec!["ion", "noi", "aba"].into_iter().map(|x| String::from(x)).collect()));
    println!("Hello, world!");
}
