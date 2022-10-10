struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut l = HashMap::new();
        let mut v = HashSet::new();
        let mut st:Vec<char> = Vec::new();

        for (ind, i) in s.char_indices() {
            l.insert(i, ind);
        }

        for (ind, i) in s.char_indices() {
            if !v.contains(&i) {
                while !st.is_empty() {
                    let last = *st.last().unwrap();
                    if last > i {
                        if ind < *l.get(&last).unwrap() {
                            st.pop();
                            v.remove(&last);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                v.insert(i);
                st.push(i);
            }
        }

        st.into_iter().collect()
    }
}


fn main() {
    println!("Hello, world!");
}
