struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image.iter().map(|x| x.iter().rev().map(|&y| 1-y).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>()
    }
}


fn main() {
    println!("Hello, world!");
}
