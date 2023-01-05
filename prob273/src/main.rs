use std::collections::HashMap;

struct Solution {

}

impl Solution {
    pub fn number_to_words(num: i32) -> String {

        println!("{}", num);
        let mut n = num;
        let hard_coded = HashMap::from( [
            (1, "One"),
            (2, "Two"),
            (3, "Three"),
            (4, "Four"),
            (5, "Five"),
            (6, "Six"),
            (7, "Seven"),
            (8, "Eight"),
            (9, "Nine"),
            (10, "Ten"),
            (11, "Eleven"),
            (12, "Twelve"),
            (13, "Thirteen"),
            (14, "Fourteen"),
            (15, "Fifteen"),
            (16, "Sixteen"),
            (17, "Seventeen"),
            (18, "Eighteen"),
            (19, "Nineteen"),
            (20, "Twenty"),
            (30, "Thirty"),
            (40, "Forty"),
            (50, "Fifty"),
            (60, "Sixty"),
            (70, "Seventy"),
            (80, "Eighty"),
            (90, "Ninety")
        ]);



        let mut result = String::new();

        if n >= 1_000_000_000 {
            result.push_str(&Solution::three_digit(n / 1_000_000_000, &hard_coded));
            result.push_str(" Billion")
        }

        n %= 1_000_000_000;

        if n >= 1_000_000 {
            if result.len() > 0 {
                result.push_str(" ");
            }
            result.push_str(&Solution::three_digit(n / 1_000_000, &hard_coded));
            result.push_str(" Million");
        }

        n %= 1_000_000;

        if n >= 1_000 {
            if result.len() > 0 {
                result.push_str(" ")
            }
            result.push_str(&Solution::three_digit(n / 1_000, &hard_coded));
            result.push_str(" Thousand");
        }

        n %= 1_000;

        if n != 0 {
            if result.len() > 0 {
                result.push_str(" ");
            }
            result.push_str(&Solution::three_digit(n, &hard_coded));
        }

        return result


    }

    fn three_digit(mut n: i32, hard_coded: &HashMap<i32, &str>) -> String {
        let mut result = String::from("");
        if n >= 100 {
            result += hard_coded.get(&(n / 100)).unwrap();
            result += " Hundred";
            n %= 100;
        }
        if hard_coded.contains_key(&n) {
            if result.len() > 0 {
                result.push_str(" ");
            }
            result += hard_coded.get(&n).unwrap();
            return result;
        }
        if n == 0 {
            return result;
        }
        if result.len() > 0 {
            result.push_str(" ");
        }
        result += hard_coded.get(&(n / 10 * 10)).unwrap();
        if n != 0 {
            result += " ";
            result += hard_coded.get(&(n % 10)).unwrap();
            result
        } else {
            result
        }
    }


}

fn main() {
    println!("{}", Solution::number_to_words((1 << 30) - 1));
    println!("{}", Solution::number_to_words(i32::MAX));
    println!("{}", Solution::number_to_words(13));
    println!("{}", Solution::number_to_words(1000));
    println!("{}", Solution::number_to_words(820));
    println!("{}", Solution::number_to_words(100));



}
