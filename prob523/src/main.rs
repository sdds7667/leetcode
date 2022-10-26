struct Solution {}


impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        // solution from leetcode
        let mut cs = 0;
        let mut h = std::collections::HashMap::new();
        h.insert(0, 0usize);
        for (ind, i) in nums.iter().enumerate() {
            cs += i;
            match h.entry(cs % k) {
                std::collections::hash_map::Entry::Occupied(e) => 
                    if *e.get() < ind {
                        return true
                    }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(ind + 1);
                },
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
