
use std::collections::HashMap;

use rand::{seq::SliceRandom, rngs::ThreadRng};


struct Solution{}

struct RandomizedSet {
    element_positions: HashMap<i32, usize>,
    elements: Vec<i32>,
    rng: ThreadRng
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        return RandomizedSet{element_positions: HashMap::new(), elements: Vec::new(), rng: rand::thread_rng()};
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.element_positions.contains_key(&val) {
            return false;
        }

        self.element_positions.insert(val, self.elements.len());
        self.elements.push(val);

        return true;
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&pos) = self.element_positions.get(&val) {
            let el_len = self.elements.len();
            self.elements.swap(pos, el_len - 1);
            self.elements.pop();
            self.element_positions.remove_entry(&val);
            true
        } else {
            false
        }
    }
    
    fn get_random(&mut self) -> i32 {
        if let Some(&x) = self.elements.choose(&mut self.rng) {
            x
        } else {
            0
        }
    }
}


fn main() {
    println!("Hello, world!");
}
