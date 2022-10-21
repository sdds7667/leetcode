struct Solution {}
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut p = std::collections::HashMap::new();

        for (ind, i) in nums.iter().enumerate() {
            let e = p.entry(i).or_insert(ind);
            if *e != ind && ind - *e <= k as usize {
                return true;
            } else {
                return false;
            }
        }

        false
    }
}
fn main() {
    println!("Hello, world!");
}
