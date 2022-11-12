use std::collections::BinaryHeap;

struct MedianFinder {
    left_heap: BinaryHeap<i32>,
    right_heap: BinaryHeap<i32>,
    size: i32
}


impl MedianFinder {

    fn new() -> Self {
        MedianFinder{left_heap: BinaryHeap::new(), right_heap: BinaryHeap::new(), size: 0}
    }

    fn add_num(&mut self, num: i32) {
        self.size += 1;
        if let Some(&x) = self.left_heap.peek() {
            if x > num {
                self.left_heap.push(num)
            } else {
                self.right_heap.push(-num)
            }

            if self.left_heap.len() == self.right_heap.len() + 2 {
                self.right_heap.push(-self.left_heap.pop().unwrap())
            } else if self.right_heap.len() == self.left_heap.len() + 1 {
                self.left_heap.push(-self.right_heap.pop().unwrap())
            }
        } else {
            self.left_heap.push(num)
        }
    }

    fn find_median(&self) -> f64 {
        if self.size % 2 == 0 {
            (self.left_heap.peek().unwrap() - self.right_heap.peek().unwrap()) as f64 / 2.0
        } else {
            *self.left_heap.peek().unwrap() as f64
        }
    }
}

fn main() {
    let mut med = MedianFinder::new();
    med.add_num(1);
    med.add_num(2);
    med.add_num(3);
    println!("{}", med.find_median())

}
