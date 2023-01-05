struct Solution {}

use std::collections::HashMap;
use std::fmt::rt::v1::Count::Param;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut losses = HashMap::new();
        for i in matches {
            let mut entry = losses.entry(i[1]).or_insert(0);
            (*entry) += 1;
            if !losses.contains_key(&i[0]) {
                losses.insert(i[0], 0);
            }
        }

        let mut zero_losses = Vec::new();
        let mut one_loss = Vec::new();

        for i in losses {
            if i.1 == 0 {
                zero_losses.push(i.0);
            } else if i.1 == 1{
                one_loss.push(i.0);
            }
        }
        zero_losses.sort_unstable();
        one_loss.sort_unstable();

        vec![zero_losses, one_loss]
    }
}

fn main() {
    println!("Hello, world!");
}
