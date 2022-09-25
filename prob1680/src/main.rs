struct Solution{}

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut num = 1;
        let mod_value = 1000000007;
        
        for i in 1..=n {
            let log_length : f64 = i.try_into().unwrap();
            let size : i32 = (log_length.log2().floor() as i32) + 1;
            num = ((num >> size) + i) % mod_value;
        }

        num
    }
}

fn main() {
    println!("Hello, world!");

    
}
