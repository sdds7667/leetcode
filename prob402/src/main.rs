struct Solution {}
impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        if k as usize == num.len() {
            return String::from("0");
        }
        let mut s: Vec<char> = Vec::new();
        for i in num.chars() {
            while let Some(&x) = s.last() {
                if x <= i || k == 0 {
                    break;
                }
                k -= 1;
                s.pop();
            }
            if s.len() > 0 || i != '0' {
                s.push(i);
            }
        }

        while k > 0 {
            k -= 1;
            s.pop();
        }
        let res: String = s.iter().collect();
        if res.len() == 0 {
            String::from("0")
        } else {
            res
        }
    }
}

fn main() {}
