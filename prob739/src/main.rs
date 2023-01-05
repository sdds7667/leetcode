struct Solution;

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut a = [0usize; 100];
        for i in (0..temperatures.len()).rev() {
            let t = temperatures[i] as usize;
            temperatures[i] = (a[t] - i) as i32;
            for j in 30..(t as usize) {
                a[j] = i;
            }
        }
        temperatures
    }
}

fn main() {
    println!("Hello, world!");
}
