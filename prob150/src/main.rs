struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut numbers_stack = Vec::new();
        for i in tokens.iter() {
            if i == "+" {
                let n2 = numbers_stack.pop().unwrap();
                let n1 = numbers_stack.pop().unwrap();
                numbers_stack.push(n1+n2);
            } else if i == "-" {
                let n2 = numbers_stack.pop().unwrap();
                let n1 = numbers_stack.pop().unwrap();
                numbers_stack.push(n1-n2);
            } else if i == "*" {
                let n2 = numbers_stack.pop().unwrap();
                let n1 = numbers_stack.pop().unwrap();
                numbers_stack.push(n1*n2);
            } else if i == "/" {
                let n2 = numbers_stack.pop().unwrap();
                let n1 = numbers_stack.pop().unwrap();
                numbers_stack.push(n1/n2);
            } else {
                numbers_stack.push(i.parse().unwrap());
            }
        }
        return numbers_stack.pop().unwrap();
    }
}

fn main() {
    println!("Hello, world!");
}
