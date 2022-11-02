use std::collections::{HashSet, VecDeque};

struct Solution {}

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        //bfs

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(&start);

        let mut its = 0;
        while !queue.is_empty() {
            let sz = queue.len();
            for i in 0..sz {
                let current = queue.pop_front().unwrap();
                if visited.contains(&current) {
                    continue;
                }
                if *current == end {
                    return its;
                }
                for gene in bank.iter() {
                    if visited.contains(gene) {
                        continue;
                    }
                    if Solution::compatible(current, gene) {
                        queue.push_back(gene);
                    }
                }
                visited.insert(current);
            }
            its += 1;
        }
        -1
    }

    pub fn compatible(l: &String, r: &String) -> bool {
        let mut diff = 0;
        let mut lit = l.chars();
        let mut rit = r.chars();
        while let Some(x) = lit.next() {
            if rit.next().unwrap() != x {
                diff += 1;
            }
        }
        diff == 1
    }
}

fn main() {

    println!("{}", Solution::min_mutation(String::from("AACCGGTT"), String::from("AAACGGTA"), vec![String::from("AACCGGTA"), String::from("AACCGCTA"), String::from("AAACGGTA")]));
    println!("Hello, world!");
}
