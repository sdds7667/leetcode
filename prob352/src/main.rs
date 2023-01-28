use std::collections::HashMap;


struct DisjointSet {
    parents: HashMap<i32, i32>,
    max: HashMap<i32, i32>,
}

impl DisjointSet {
    fn new() -> Self {
        Self {
            parents: HashMap::new(),
            max: HashMap::new(),
        }
    }
    fn merge(&mut self, value: i32) -> bool {
        if let Some(_) = self.find(value) {
            return false;
        }
        let mut current_parent = value;
        let mut max_value = value;

        if let Some(left_parent) = self.find(value - 1) {
            current_parent = left_parent;
        }
        if let Some(right_parent) = self.find(value + 1) {
            self.parents.insert(right_parent, current_parent);
            max_value = *(self.max.get(&right_parent).unwrap());
        }

        self.max.insert(current_parent, max_value);
        self.parents.insert(value, current_parent);
        if current_parent != value {
            self.max.insert(value, max_value);
        }
        true
    }

    fn find(&mut self, value: i32) -> Option<i32> {
        return if let Some(&parent) = self.parents.get(&value) {
            if parent == value {
                Some(parent)
            } else {
                let new_parent = self.find(parent).unwrap();
                self.parents.insert(value, new_parent);
                Some(new_parent)
            }
        } else {
            None
        };
    }
}

struct SummaryRanges {
    ranges: Vec<i32>,
    disjoint_set: DisjointSet,
    sorted: bool,
}


/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges {
            ranges: Vec::new(),
            disjoint_set: DisjointSet::new(),
            sorted: true,
        }
    }

    fn add_num(&mut self, value: i32) {
        if self.disjoint_set.merge(value) {
            self.ranges.push(value);
        }
        self.sorted = false;
    }

    fn get_intervals(&mut self) -> Vec<Vec<i32>> {
        if !self.sorted {
            self.sorted = true;
            self.ranges.sort();
        }
        let mut result = Vec::new();

        let mut new_intervals = Vec::new();
        for &i in self.ranges.iter() {
            if self.disjoint_set.find(i).unwrap() == i {
                new_intervals.push(i);
                result.push(vec![i, *self.disjoint_set.max.get(&i).unwrap()])
            }
        }

        return result;
    }
}


struct Solution;

fn main() {
    println!("Hello, world!");
}
