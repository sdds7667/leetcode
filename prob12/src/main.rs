
struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::new();
        
        let map = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I")
        ];
        let mut i = i32::MAX;
        let mut st = "";
        let mut it = map.iter();
        while num > 0 {
            if i > num {
                let t = it.next().unwrap();
                st = t.1;
                i = t.0;
            } else {
                num -= i;
                result.push_str(st);
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
