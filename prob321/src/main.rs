use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let min_left = if k > nums2.len() { k - nums2.len() } else { 0 };
        let mut best_vec = Merge::sort(
            Solution::reduce_array_to_k(&nums1, min_left),
            Solution::reduce_array_to_k(&nums2, k - min_left));
        for i in (min_left + 1)..=min((k - min_left), nums1.len()) {
            best_vec = max(best_vec, Merge::sort(
                Solution::reduce_array_to_k(&nums1, i),
                Solution::reduce_array_to_k(&nums2, k - i),
            ));
        }
        best_vec
    }

    pub fn reduce_array_to_k(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let mut result = Vec::new();
        let mut n = nums.len();
        for i in 0..nums.len() {
            let c = nums[i];
            while let Some(&x) = result.last() {
                if x < c && result.len() + n - i > k {
                    result.pop();
                } else {
                    break;
                }
            }

            if result.len() != k {
                result.push(c);
            }
        }

        return result;
    }

    pub fn min_max(num1: Vec<i32>, num2: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        for i in 0..(min(num1.len(), num2.len())) {
            if num1[i] == num2[i] {
                continue;
            } else if num1[i] > num2[i] {
                return (num2, num1);
            } else {
                return (num1, num2);
            }
        }
        if num1.len() > num2.len() {
            (num2, num1)
        } else {
            (num1, num2)
        }
    }
}

mod Merge {
    use std::cmp::Ordering;

    pub fn sort(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut best = Vec::new();
        let mut visited = vec![false; (nums1.len() + 1) * (nums2.len() + 1)];
        backtrack(&mut visited, &nums1, &nums2, &mut result, &mut best, 0, 0);
        return best;
    }

    pub fn max(old_best: &mut Vec<i32>, candidate: &Vec<i32>) {
        if old_best.len() < candidate.len() {
            for &i in candidate.iter() {
                old_best.push(i);
            }
            return;
        }
        for i in 0..old_best.len() {
            if old_best[i] == candidate[i] {
                continue;
            } else if old_best[i] < candidate[i] {
                old_best.clone_from(candidate);
            } else {
                return;
            }
        }
    }

    pub fn backtrack(visited: &mut Vec<bool>,
                     nums1: &Vec<i32>,
                     nums2: &Vec<i32>,
                     result: &mut Vec<i32>,
                     best: &mut Vec<i32>, i: usize, j: usize) {
        if i == nums1.len() {
            if j == nums2.len() {
                max(best, result);
                return;
            }
        }

        let n1 = if i == nums1.len() { i32::MIN } else { nums1[i] };
        let n2 = if j == nums2.len() { i32::MIN } else { nums2[j] };

        match n1.cmp(&n2) {
            Ordering::Equal => {
                result.push(n1);
                backtrack(visited, nums1, nums2, result, best, i + 1, j);
                backtrack(visited, nums1, nums2, result, best, i, j + 1);
                result.pop();
            }
            Ordering::Greater => {
                result.push(n1);
                backtrack(visited, nums1, nums2, result, best, i + 1, j);
                result.pop();
            }
            Ordering::Less => {
                result.push(n2);
                backtrack(visited, nums1, nums2, result, best, i, j + 1);
                result.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Merge, Solution};

    #[test]
    fn test_1() {
        let nums1 = vec![8, 9];
        let nums2 = vec![3, 9];
        let k = 3;
        assert_eq!(vec![9, 8, 9], Solution::max_number(nums1, nums2, k));
    }

    #[test]
    fn test_2() {
        let nums1 = vec![6, 7];
        let nums2 = vec![6, 0, 4];
        let k = 5;
        assert_eq!(vec![6, 7, 6, 0, 4], Solution::max_number(nums1, nums2, k));
    }

    #[test]
    fn test_3() {
        let num1 = vec![5,7,7,0,1,6,7,2,2,4,6,8,9,2,0,9,8,7,6,3,9,4,8,8,4,5,3,3,7,4,3,2,8,9,8,4,0,2,0,2,2,0,4,2,2,8,6,7,1,0,8,7,5,4,6,4,1,7,4,4,3,7,5,8,8,0,3,1,3,4,6,0,6,9,6,6,4,2,1,9,3,7,4,4,4,2,1,9,5,2,1,7,6,0,1,3,5,3,7,7];
        let num2 = vec![8,3,7,8,6,9,1,5,5,0,5,2,8,7,8,3,3,7,9,2];
        let k = 100;
        assert_eq!(vec![9,9,9,8,8,8,7,8,6,9,4,5,3,3,7,4,3,2,8,9,8,4,1,5,5,0,5,2,8,7,8,3,3,7,9,2,0,2,0,2,2,0,4,2,2,8,6,7,1,0,8,7,5,4,6,4,1,7,4,4,3,7,5,8,8,0,3,1,3,4,6,0,6,9,6,6,4,2,1,9,3,7,4,4,4,2,1,9,5,2,1,7,6,0,1,3,5,3,7,7],
            Solution::max_number(num1, num2, k));
    }


    #[test]
    fn test_sort() {
        let nums1 = vec![6, 0, 4];
        let nums2 = vec![6, 7];
        assert_eq!(vec![6, 7, 6, 0, 4], Merge::sort(nums1, nums2));

        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4];
        assert_eq!(vec![4, 1, 3, 4, 1, 2], Merge::sort(nums1, nums2));

        let nums2 = vec![4, 1, 2];
        let nums1 = vec![1, 3, 4];
        assert_eq!(vec![4, 1, 3, 4, 1, 2], Merge::sort(nums1, nums2));
    }
}

fn main() {
    let nums1 = vec![6, 7];
    let nums2 = vec![6, 0, 4];
    let k = 5;
    println!("{:?}", Solution::max_number(nums1, nums2, k));
    println!("Hello, world!");
}
