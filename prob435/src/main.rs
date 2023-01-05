struct Solution {

}

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|x| x[1]);
        let mut time = i32::MIN;
        let mut result = 0;
        for i in intervals {
            if time <= i[0] {
                time = i[1];
            } else {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
