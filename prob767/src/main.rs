struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut count_map = [[0usize; 2]; 26];
        for i in 0..26 {
            count_map[i][0] = i;
        }
        for i in s.into_bytes() {
            count_map[i as usize - 97][1] += 1;
        }
        count_map.sort_unstable_by_key(|x| 1000 - x[1]);
        let mut v = vec![String::new(); count_map[0][1]];
        let mut vi = 0;
        let n = v.len();
        for mut i in count_map {
            while i[1] > 0 {
                i[1] -= 1;
                v[vi].push(char::from(i[0] as u8 + 97));
                vi = (vi + 1) % n;
            }
        }
        if n > 1 {
            for i in 0..(v.len() - 1) {
                if v[i].len() == 1 {
                    return String::new();
                }
            }
        }
        v.join("")
    }
}

fn main() {
    println!("{}", Solution::reorganize_string(String::from("aab")));
}
