use std::cmp::Ordering;
use std::collections::HashMap;

struct UF {
    parent_mapping: HashMap<char, char>
}

impl UF {
    pub fn new() -> Self {
        Self {
            parent_mapping: HashMap::new()
        }
    }

    pub fn find(&mut self, c: char) -> char {
        let entry = self.parent_mapping.entry(c).or_insert(c);
        if *entry == c {
            return c;
        }
        let j = *entry;
        let result = self.find(j);
        self.parent_mapping.insert(c, result);
        return result;
    }

    pub fn merge(&mut self, a: char, b: char) -> bool {
        let mut ap = self.find(a);
        let mut bp = self.find(b);
        match ap.cmp(&bp) {
            Ordering::Less => {
                self.parent_mapping.insert(bp, ap);
                true
            }
            Ordering::Equal => { false }
            Ordering::Greater => {
                self.parent_mapping.insert(ap, bp);
                true
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut uf = UF::new();
        let mut v1: Vec<_> = s1.chars().collect();
        let mut v2: Vec<_> = s2.chars().collect();
        for i in 0..v1.len() {
            uf.merge(v1[i], v2[i]);
        }
        base_str.chars().map(|x| uf.find(x)).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
