
struct Solution{}

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let sequence_bytes = sequence.as_bytes();
        let word_bytes = word.as_bytes();
        let mut wi = 0;
        let wbl = word_bytes.len();
        for i in 0..sequence_bytes.len() {
            let mut cwi = 0;
            let mut total_word_index = 0;
            for j in i..sequence_bytes.len() {
                // assume j is the first character of the repeating subsequence.
                if sequence_bytes[j] != word_bytes[cwi] {
                    break;
                }
                cwi = (cwi + 1) % wbl;
                total_word_index += 1;
            }
            wi = std::cmp::max(wi, total_word_index);
        }
        return (wi / word.len()) as i32;
    }
}

fn main() {
    println!("Hello, world!");
}
