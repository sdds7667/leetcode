struct Solution;

use std::cmp::{max, min};
use std::collections::HashSet;

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        // union find solution
        // greedy

        if n == 1 {
            return true;
        }
        let mut uf = UnionFind::new(n);
        let mut adj_list = vec![Vec::new(); (n + 1) as usize];
        for i in dislikes.iter() {
            adj_list[i[0] as usize].push(i[1] as usize);
            adj_list[i[1] as usize].push(i[0] as usize);
        }
        for k in 0..adj_list.len() {
            let i = &adj_list[k];
            if i.len() >= 2 {
                let p = i[0];
                if uf.find(k) == uf.find(p) {
                    return false;
                }
                for j in 1..i.len() {
                    if uf.find(k) == uf.find(i[j]) {
                        return false;
                    }
                    uf.merge(p, i[j]);
                }
            }
        }
        true
    }
}

struct UnionFind {
    rank: Vec<i32>,
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: i32) -> Self {
        let parent: Vec<usize> = (0..=(n as usize)).collect();
        UnionFind { rank: vec![0; (n + 1) as usize], parent }
    }

    pub fn find(self: &mut Self, a: usize) -> usize {
        if self.parent[a] == a {
            return a;
        }
        self.parent[a] = self.find(self.parent[a]);
        return self.parent[a];
    }

    pub fn cluster_count(self) -> i32 {
        let mut i = 0;
        let mut visited = HashSet::new();
        for j in 1..self.parent.len() {
            if !visited.contains(&self.parent[j]) {
                i += 1;
                if i == 3 {
                    return i;
                }
                visited.insert(j);
            }
        }
        i
    }


    pub fn merge(self: &mut Self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb {
            return false;
        } else {
            if self.rank[pa] > self.rank[pb] {
                self.parent[pb] = pa;
            } else if self.rank[pb] > self.rank[pa] {
                self.parent[pa] = pb;
            } else {
                self.parent[pb] = pa;
                self.rank[pa] += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = [[1, 2], [3, 4], [5, 6], [6, 7], [8, 9], [7, 8]];
        let mut y: Vec<_> = v.iter().map(|x| vec![x[0], x[1]]).collect();
        assert_eq!(true, Solution::possible_bipartition(10, y));
    }
}

fn main() {
    // Solution::possible_bipartition(4, vec![vec![1, 2], vec![3, 4], vec![1, 3], vec![1, 4]]);
    Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]);
}
