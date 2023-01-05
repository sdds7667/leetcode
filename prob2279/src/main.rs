struct Solution;


impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, mut additional_rocks: i32) -> i32 {
        let mut bags = Vec::new();
        for i in 0..capacity.len() {
            bags.push(capacity[i] - rocks[i]);
        }
        bags.sort();
        let mut c = 0;
        for i in bags {
            additional_rocks -= i;
            if additional_rocks >= 0 {
                c += 1;
            } else {
                break;
            }
        }
        return c;
    }
}


fn main() {
    println!("Hello, world!");
}
