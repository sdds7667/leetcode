struct Solution {

}

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut result = String::new();
        for i in address.chars() {
            if i == '.' {
                result.push('[');
                result.push('.');
                result.push(']');
            } else {
                result.push(i);
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
