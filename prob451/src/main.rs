
struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut count = std::collections::HashMap::new();
        for i in s.chars(){
            (*count.entry(i).or_insert(0)) += 1;
        }
        let mut char_array = count.iter().map(|x|
            ((0..*x.1).map(|_| *x.0).collect::<String>())).collect::<Vec<String>>();
        char_array.sort_unstable_by(|l, r| r.len().cmp(&l.len()));
        char_array.join("")
    }
}

fn main() {
    println!("Hello, world!");
}
