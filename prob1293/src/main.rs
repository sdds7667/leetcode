use std::collections::{hash_map::Entry, HashMap, VecDeque};

struct Solution {}

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let h = grid.len() as i32;
        let w = grid[0].len() as i32;
        let dir = vec![(1, 0), (0, 1), (0, -1), (-1, 0)];
        let mut v: HashMap<(i32, i32), i32> = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((0, 0, k));
        let mut steps = 0;

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (y, x, k) = q.pop_front().unwrap();
                for (dy, dx) in dir.iter() {
                    let ny: i32 = dy + y;
                    let nx: i32 = dx + x;
                    if ny == h - 1 && nx == w - 1 {
                        return steps + 1;
                    } else {
                        if (ny == -1 || nx == -1 || ny == h || nx == w) {
                            continue;
                        } else {
                            let k = if grid[ny as usize][nx as usize] == 1 {
                                k - 1
                            } else {
                                k
                            };
                            if k == -1 {
                                continue;
                            }
                            match v.entry((ny, nx)) {
                                Entry::Occupied(mut e) => {
                                    let e = e.get_mut();
                                    if *e < k {
                                        *e = k;
                                        q.push_back((ny, nx, k));
                                    }
                                }
                                Entry::Vacant(_) => q.push_back((ny, nx, k)),
                            };
                        }
                    }
                }
                v.insert((y, x), k);
            }
            steps += 1
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}
