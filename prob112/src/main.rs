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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(x) => {
                let v = x.borrow().val;
                match (x.borrow().left.clone(), x.borrow().right.clone()) {
                    (None, None) => target_sum == v,
                    (Some(x), None) | (None, Some(x)) => 
                        Solution::has_path_sum(Some(x), target_sum - v),
                    (Some(x), Some(y)) => 
                        Solution::has_path_sum(Some(x), target_sum - v) || Solution::has_path_sum(Some(y), target_sum - v)
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
