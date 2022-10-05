struct Solution {}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::collections::VecDeque;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let None = root {
            return Vec::new();
        }

        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while queue.len() != 0 {
            let mut r = Vec::new();
            let ln = queue.len();
            for _ in 0..ln {
                let rc = queue.pop_front().unwrap().unwrap();
                let ref_cell = (*rc).borrow();
                r.push(ref_cell.val);

                match (ref_cell.left.clone(), ref_cell.right.clone()) {
                    (None, None) => {continue},
                    (Some(x), None) | (None, Some(x)) => {queue.push_back(Some(x))},
                    (Some(x), Some(y)) => {
                        queue.push_back(Some(x));
                        queue.push_back(Some(y));
                    }
                };
            }
            res.push(r);
        }
        return res;
    }
}

fn main() {
    println!("Hello, world!");
}
