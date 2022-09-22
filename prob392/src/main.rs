struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut si: usize = 0;

        let sit = s.len();

        let s = s.as_bytes();
        let t = t.as_bytes();

        for &c in t {
            if s[si] == c {
                si += 1;
                if si == sit {
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    println!("Hello, world!");
}
