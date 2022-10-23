use std::collections::{hash_map::Entry, HashMap};

struct Solution {}

/*
Given two strings s and t of lengths m and n respectively,
return the minimum window substring of s such that every
character in t (including duplicates) is included in the window.
If there is no such substring, return the empty string "".

The testcases will be generated such that the answer is unique.

A substring is a contiguous sequence of characters within the string.
 */

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return String::from("");
        }
        let mut len = usize::MAX;
        let mut ml = 0usize;
        let mut mr = 0usize;

        let mut m = HashMap::new();

        for &i in t.as_bytes().iter() {
            let e = m.entry(i).or_insert(0);
            *e = *e + 1;
        }

        let mut l = 0;
        let mut r = 0;
        let s1 = s.as_bytes();
        let mut f : bool;

        while r < s1.len() {
            f = false;
            while r < s1.len() {
                match m.entry(s1[r]) {
                    Entry::Occupied(mut e) => {
                        r += 1;
                        let e = e.get_mut();
                        *e -= 1;
                        if *e <= 0 {
                            if !m.iter().any(|x| *x.1 > 0) {
                                if r - l < len {
                                    println!("Length set to {}", r-l);
                                    mr = r;
                                    ml = l;
                                    len = r - l;
                                }
                                f = true;
                                break;
                            }
                        }
                    }
                    Entry::Vacant(_) => r += 1,
                };
            }

            if !f {
                break;
            }
            // println!("Expanding {}", s.chars().skip(l).take(r-l).collect::<String>());

            while l < r {
                match m.entry(s1[l]) {
                    Entry::Occupied(mut e) => {
                        let e = e.get_mut();
                        *e += 1;
                        l += 1;
                        if *e > 0 {
                            break;
                        } else {
                            // println!("Updating the length to {}", r-l);
                            if r - l < len {
                                mr = r;
                                ml = l;
                                len = r - l;
                            }
                        }
                    }
                    Entry::Vacant(_) => {
                        l += 1;
                        if r - l < len {
                            // println!("Character is vacant. ");
                            mr = r;
                            ml = l;
                            len = r - l;
                        }
                    }
                }
            }
            // println!("Compressed to {}", s.chars().skip(l).take(r-l).collect::<String>());
        }

        println!("The length of the string {}", len);
        if len == usize::MAX {
            String::new()
        } else {
            // println!("Returning from {} tp {}", ml, mr);
            s.chars().skip(ml).take(mr - ml).collect()
        }
    }
}

#[test]
fn leetcode() {
    let tests = vec![
        ("ADOBECODEBANC", "ABC", "BANC"),
        ("a", "a", "a"),
        ("a", "aa", ""),
        ("ab", "a", "a"),
        ("aa", "a", "a")
    ];

    for (s1, s2, ex) in tests {
        let s1 = String::from(s1);
        let s2 = String::from(s2);
        let ex = String::from(ex);
        assert_eq!(ex, Solution::min_window(s1, s2));
    }
}

fn main() {
    println!("Hello, world!");
}
