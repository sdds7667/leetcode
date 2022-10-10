struct Solution {}

impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        /*
         * if all A: change last to b
         * aaa -> aab
         *
         * Rule: First Non-A char goes to a
         * zaz -> aaz
         * abccba -> aaccba
         * abzba -> aazba
         *
         * Exception: first-non-a is in the middle of the palindrome:
         * aazaa -> aazab
         *
         * Algorithm:
         *
         * if string length is even:
         *  if string is only a's:
         *      convert last a to b
         *  else:
         *      convert first non-a to a
         * else:
         *  if string length is 1:
         *      return empty string
         *  else:
         *      m <- middle index of the string (<i32> string length / 2); (aka) | m <- 3/2 = 1
         *      if string only a's:
         *          convert last a to b
         *      else:
         *          if first non a's index is m:
         *              convert last a to b
         *          else
         *              convert first non a to a
         *
         *
         *
         */

        if palindrome.len() <= 1 {
            return String::from("");
        }

        let str_len = palindrome.len() as i32;
        if str_len % 2 == 0 {
            let mut change_index = -1;
            for (ind, i) in palindrome.char_indices() {
                if i != 'a' {
                    change_index = ind as i32;
                    break;
                }
            }
            if change_index == -1 {
                palindrome.pop();
                palindrome.push('b');
            } else {
                palindrome.replace_range(change_index as usize..change_index as usize + 1, "a");
            }
        } else {
            let mut change_index = usize::MAX;
            let middle_index = palindrome.len() / 2;
            for (ind, i) in palindrome.char_indices() {
                if i != 'a' {
                    if ind == middle_index {
                        break;
                    } else {
                        change_index = ind;
                        break;
                    }
                }
            }
            if change_index == usize::MAX {
                palindrome.pop();
                palindrome.push('b');
            } else {
                palindrome.replace_range(change_index..change_index+1, "a");
            }
        }
        palindrome
    }
}

fn main() {
    println!("Hello, world!");
}
