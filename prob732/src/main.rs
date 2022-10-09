use std::{collections::{HashMap, BTreeMap}, cmp::max};


struct MyCalendarThree {
    diff: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        MyCalendarThree{diff: BTreeMap::new()}
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        let mut v = 1;
        if let Some(x) = self.diff.get(&start) {
            v += x;
        }
        self.diff.insert(start, v);

        let mut v = -1;
        if let Some(&x) = self.diff.get(&end) {
            v += x;
        }
        self.diff.insert(end, v);

        
        let mut best = 0;
        let mut current = 0;
        
        for (_, &v) in self.diff.iter() {
            current += v;
            best = max(current, best);
        }

        return best;
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */


fn main() {
    println!("Hello, world!");
}
