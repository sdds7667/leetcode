struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {

        let mut levels = vec![0; edges.len()];
        let mut res = -1;
        for i in 0..(edges.len() as i32){
            res = res.max(Solution::dfs(&edges, &mut levels, 1, i));
        }
        return res;
    }

    pub fn dfs(edges: &Vec<i32>, levels: &mut Vec<i32>, level: i32, node: i32) -> i32 {
        if node < 0 {
            return -1;
        }
        let node = node as usize;
        return if edges[node] == -1 || levels[node] == -1 {
            -1
        } else if levels[node] > 0 {
            let old_level = levels[node];
            levels[node] = -1;
            level - old_level
        } else {
            levels[node] = level;
            let res = Solution::dfs(edges, levels, level + 1, edges[node]);
            levels[node] = -1;
            res
        }
    }
}

fn main() {
    println!("Hello, world!");
}
