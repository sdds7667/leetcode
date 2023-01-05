use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        // since we can swap two existing characters, the order doesn't matter
        // the rule about transforming every occurence means that
        // a bb ccc
        // c aa bbb // a cc bbb // a bb ccc
        // we need to have the same counts.


        // aa bb ccc
        // aaa bb c d // a bb ccc d. Cannot get aa from a d

        // aa b
        // cc b / dd c -> cannot get a, since it's missing

        // solution:
        // create sets of characters from both strings.
        // if the sets are different, the strings are not close

        // if the counts sorted are different, cannot convert:
        // a d -> aa case
        let zero_byte = 'a' as usize;
        let mut counts = [[0; 26]; 2];
        for i in word1.bytes() {
            counts[0][i as usize - zero_byte] += 1;
        }
        for i in word2.bytes() {
            counts[1][i as usize - zero_byte] += 1;
        }

        println!("{counts:?}");
        println!("{:?}", counts[0]);
        println!("{:?}", counts[1]);
        // compare the hash differences
        for i in 0..26 {
            if counts[0][i] == 0 {
                println!("Count is 0 for {i}");
                println!("Count {}", counts[1][i]);
                if !counts[1][i] == 0 {
                    return false;
                }
            }
            if counts[1][i] == 0 {
                println!("Count is 0 for {i} (2)");
                if !(counts[0][i] == 0) {
                    return false;
                }
            }
        }

        // sorting the two arrays should result in equal arrays
        counts[0].sort();
        counts[1].sort();
        return counts[0] == counts[1];
    }
}

fn main() {
    Solution::close_strings(String::from("uau"), String::from("ssx"));
}
