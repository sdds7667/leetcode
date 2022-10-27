use std::cmp::{max, min};
struct Solution {}
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len() as i32;
        let mut m = 0;
        for y in ((-n) + 1..n).rev() {
            for x in ((-n) + 1..n).rev() {
                m = max(m, Solution::overlap(&img1, &img2, n, x, y))
            }
        }
        m
    }

    pub fn overlap(img1: &Vec<Vec<i32>>, img2: &Vec<Vec<i32>>, n: i32, x: i32, y: i32) -> i32 {
        // println!("Shift is {}, {}", x, y);
        let mut l1 = 0usize;
        let mut l2 = 0usize;
        let mut r1 = n as usize;
        let mut r2 = n as usize;

        let mut t1 = 0usize;
        let mut t2 = 0usize;
        let mut b1 = n as usize;
        let mut b2 = n as usize;

        if x > 0 {
            r1 -= x as usize;
            l2 += x as usize;
        } else {
            l1 = l1 + (-x) as usize;
            r2 = r2 - (-x) as usize;
        }

        if y > 0 {
            b1 -= y as usize;
            t2 += y as usize;
        } else {
            t1 = t1 + (-y) as usize;
            b2 = b2 - (-y) as usize;
        }
        // println!("Bounds, arr1: ({},{}) -> ({},{}); arr2: ({},{}) -> ({},{})", l1, t1, r1, b1, l2, t2, r2, b2);
        let mut o = 0;
        for yi in 0..min(b1, b2) {
            for xi in 0..min(r1, r2) {
                if img1[yi + t1][xi + l1] == 1 && img2[yi + t2][xi + l2] == 1 {
                    o += 1;
                }
            }
        }
        o
    }
}
fn main() {
    println!("Hello, world!");
}
