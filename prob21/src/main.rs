
 #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution{}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(x),
            (Some(mut x), Some(mut y)) => {
                if x.val > y.val {
                    y.next = Solution::merge_two_lists(Some(x), y.next);
                    Some(y)
                } else {
                    x.next = Solution::merge_two_lists(x.next, Some(y));
                    Some(x)
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
