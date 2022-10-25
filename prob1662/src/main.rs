struct Solution {}


impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {

        let word1 = word1.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
        let word2 = word2.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();

        let mut w1 = 0usize;
        let mut w2 = 0usize;

        let mut c1 = 0usize;
        let mut c2 = 0usize;

        let mut n1 = word1.len();
        let mut n2 = word2.len();

        while w1 < n1 && w2 < n2 {
            if word1[w1][c1] != word2[w2][c2] {
                return false;
            }
            c1 += 1;
            c2 += 1;

            if c1 == word1[w1].len() {
                c1 = 0;
                w1 += 1;
            }

            if c2 == word2[w2].len() {
                c2 = 0;
                w2 += 1;
            }
        }


        w1 == n1 && w2 == n2
    }
}

fn main() {
    println!("Hello, world!");
}
