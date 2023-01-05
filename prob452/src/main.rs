struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|x| x[1]);
        let mut balloon_index = 0;
        let mut arrows = 0;
        loop {
            let arrow_x = points[balloon_index][1];
            arrows += 1;
            loop {
                if balloon_index == points.len() {
                    return arrows;
                }
                if points[balloon_index][0] <= arrow_x {
                    balloon_index += 1;
                } else {
                    break;
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
