struct Solution{}

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut sm : HashMap<u8, u8> = HashMap::new();
        let mut tm : HashMap<u8, u8> = HashMap::new();
        let s = s.as_bytes();
        let t = t.as_bytes();


        for i in 0..s.len() {
            let sc = s[i];
            let tc = t[i];

            let scm = sm.get(&sc);
            let tcm = tm.get(&tc);

            if let Some(&r) = scm {
                if r != tc {
                    return false;
                }
            } else {
                if let Some(_) = tcm {
                    return false;
                }
                sm.insert(sc, tc);
                tm.insert(tc, sc);
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
    let s1: String = String::from("egg");
    let s2: String = String::from("add");
    println!("{}", Solution::is_isomorphic(s1, s2));


    println!("{}", Solution::is_isomorphic(String::from("foo"), String::from("bar")));
}
