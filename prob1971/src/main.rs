use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut adjacency_list = vec![Vec::new(); n as usize];
        for i in edges {
            adjacency_list[i[0] as usize].push(i[1] as usize);
            adjacency_list[i[1] as usize].push(i[0] as usize);
        }
        let destination = destination as usize;
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n as usize];
        queue.push_back(source as usize);
        while !queue.is_empty(){
            for _ in 0..queue.len() {
                let i = queue.pop_front().unwrap();
                if destination == i {
                    return true;
                }
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                for &j in adjacency_list[i].iter() {
                    queue.push_back(j);
                }
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
