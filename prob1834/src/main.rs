struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;


#[derive(Eq, PartialEq)]
struct Task {
    index: usize,
    enqueue_time: i32,
    processing_time: i32,
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = other.processing_time.cmp(&self.processing_time);
        if cmp == Ordering::Equal {
            other.index.cmp(&self.index)
        } else {
            cmp
        }
    }
}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks_vector: Vec<_> = tasks.iter().enumerate().map(|(i, x)| Task {
            index: i,
            enqueue_time: x[0],
            processing_time: x[1],
        }).collect();

        tasks_vector.sort_unstable_by(|x, y| {
            let cmp = x.enqueue_time.cmp(&y.enqueue_time);
            if cmp == Ordering::Equal {
                let cmp = x.processing_time.cmp(&y.processing_time);
                if cmp == Ordering::Equal {
                    x.index.cmp(&y.index)
                } else {
                    cmp
                }
            } else {
                cmp
            }
        });
        let mut result = Vec::new();
        let mut i = 0;


        let n = tasks.len();
        let mut bh = BinaryHeap::new();
        while i < n {
            bh.push(&tasks_vector[i]);
            let mut t = tasks_vector[i].enqueue_time;
            i += 1;
            while let Some(x) = bh.pop() {
                result.push(x.index as i32);
                for j in i..n {
                    if tasks_vector[j].enqueue_time <= t + x.processing_time {
                        bh.push(&tasks_vector[j]);
                        if j == n - 1 {
                            i = j + 1;
                            break;
                        }
                    } else {
                        i = j;
                        break;
                    }
                }
                t += x.processing_time;
            }
        }
        return result;
    }
}


fn main() {
    let tasks = vec![vec![7, 10], vec![7, 12], vec![7, 5], vec![7, 4], vec![7, 2]];
    println!("{:?}", Solution::get_order(tasks));
}
