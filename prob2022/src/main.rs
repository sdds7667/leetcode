struct Solution {}

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if (original.len() as i32) != (m * n) {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(m as usize);
        let mut i = 0;
        for _ in 0..m {
            let mut tmp = Vec::with_capacity(n as usize);
            for _ in 0..n {
                tmp.push(original[i]);
                i += 1;
            }
            res.push(tmp);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
