struct MyQueue {
    v1: Vec<i32>,
    v2: Vec<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue{v1:  Vec::new(),
        v2: Vec::new()}
    }
    
    fn push(&self, x: i32) {
        while self.v1.len() > 0 {
            self.v2.push(self.v1.pop().unwrap())
        }
        self.v2.push(x);
        while self.v2.len() > 0 {
            self.v1.push(self.v2.pop().unwrap())
        }
    }
    
    fn pop(&self) -> i32 {
        self.v1.pop().unwrap()
    }
    
    fn peek(&self) -> i32 {
        *self.v1.back().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.v1.len() == 0
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

fn main() {
    println!("Hello, world!");
}
