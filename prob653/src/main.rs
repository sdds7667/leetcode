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
      right: None
    }
  }
}
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut values = HashMap::new();
        match root {
            Some(x) => Solution::collectIntoMap(x, &mut values, k),
            None => false
        }
    }

    pub fn collectIntoMap(root: Rc<RefCell<TreeNode>>, map: &mut HashMap<i32, i32>, target: i32) -> bool {

        let cv = root.borrow().val;
        if cv * 2 == target {
            if let Some(&x) = map.get(&cv) {
                return x > 1;
            }
        } else if let Some(x) = map.get(&(target - root.borrow().val)) {
            return true
        } 

        let mut count = 1;
        if let Some(x) = map.get(&cv) {
            count += x;
        } 
        map.insert(cv, count);

        let l = root.borrow().left.clone();
        let r = root.borrow().right.clone();
        match (l, r) {
            (None, None) => false,
            (None, Some(x)) | (Some(x), None) => Solution::collectIntoMap(x,map,target),
            (Some(x), Some(y)) => Solution::collectIntoMap(x, map, target) || Solution::collectIntoMap(y, map, target),
        }
    }


}


fn main() {
    println!("Hello, world!");
}
