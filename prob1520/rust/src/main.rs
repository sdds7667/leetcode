use std::cmp::{max, min};

struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        let char_array = s.as_bytes();

        let mut fc = [usize::MAX; 123];
        let mut lc = [0usize; 123];
        // step 1: collect indices

        for (index, &chr) in char_array.iter().enumerate() {
            let chr = chr as usize;
            fc[chr] = min(fc[chr], index);
            lc[chr] = index;
        }

        let mut intervals = Vec::new();

        for (index, i) in fc.iter().enumerate() {
            let mut r = lc[index];
            let mut j = *i;
            while j < r {
                if fc[char_array[j] as usize] < *i {
                    break;
                }
                r = max(r, lc[char_array[j] as usize]);
                j += 1;
            }
            if j == r {
                intervals.push((*i, r));
            }
        }

        intervals.sort_unstable_by_key(|x| x.1);
        let mut res = Vec::new();
        let mut i = 0;
        for (st, f) in intervals {
            if st < i {
                continue;
            }
            i = f;
            let mut sbs = String::new();
            for j in st..=f {
                sbs.push(char::from(char_array[j]));
            }
            res.push(sbs);
        }

        res
    }
}

fn main() {
    Solution::max_num_of_substrings(String::from("adefaddaccc"));
}
