use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let ni = n;
        let n = n as usize;
        let mut adj_list = vec![Vec::new(); n];
        for i in edges {
            adj_list[i[0] as usize].push(i[1] as usize);
            adj_list[i[1] as usize].push(i[0] as usize);
        }
        let mut parent: Vec<usize> = vec![0; (n + 1)];
        let mut result = vec![[1, 0]; (n + 1)];
        result[n][0] = 0;
        Solution::dfs(&adj_list, &mut parent, &mut result, 0, 0);
        let mut r = vec![0; n];
        r[0] = result[0][1];
        for &i in adj_list[0].iter() {
            Solution::dfs_solve(ni, &adj_list, &mut result, &mut r, i, 0);
        }
        r
    }
    pub fn dfs_solve(ni: i32, adj_list: &Vec<Vec<usize>>, result: &mut Vec<[i32; 2]>, r: &mut Vec<i32>, i: usize, p: usize) {
        let c = result[p][0] - 2 * result[i][0] + result[p][1];
        r[i] = c;
        result[i][0] = ni;
        result[i][1] = c;
        for &child in adj_list[i].iter() {
            if child == p {
                continue;
            }
            Solution::dfs_solve(ni, adj_list, result, r, child, i);
        }
    }

    pub fn dfs(adj_list: &Vec<Vec<usize>>, parent: &mut Vec<usize>,
               res: &mut Vec<[i32; 2]>, p: usize, c: usize) {
        parent[c] = p;
        for &i in adj_list[c].iter() {
            if i == p {
                continue;
            }
            Solution::dfs(adj_list, parent, res, c, i);
            res[c][0] += res[i][0];
            res[c][1] += res[i][0] + res[i][1];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_leetcode_1() {
        let v: Vec<_> = [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]].map(|x| vec![x[0], x[1]]).into_iter().collect();
        let r: Vec<_> = vec![8, 12, 6, 10, 10, 10];
        let n = 6;
        assert_eq!(r, Solution::sum_of_distances_in_tree(6, v));
    }
}

fn main() {
    println!("Hello, world!");
}
