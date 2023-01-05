use std::collections::HashSet;

struct Solution {

}
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut vs = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let mut m = s.len() / 2;
        let mut c1 = 0;
        let mut c2 = 0;
        for i in s.char_indices() {
            if vs.contains(&i.1) {
                if i.0 < m {
                    c1 += 1;
                } else {
                    c2 += 1;
                }
            }
        }
        c1 == c2
    }
}
fn main() {
    println!("Hello, world!");
}
