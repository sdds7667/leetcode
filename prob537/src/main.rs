
struct Solution {}

struct ComplexNumber {
    x: i32,
    y: i32,
}

impl ComplexNumber {
    pub fn from(num1: &String) -> Self {
        let mut sx: i32 = 1;
        let mut x: i32 = 0;

        let mut sy = 1;
        let mut y = 0;

        let mut s = 1;

        for i in num1.chars() {
            match s {
                1 | 4 => match i {
                    '-' => {
                        if s == 1 {
                            sx = -1;
                        } else {
                            sy = -1;
                        }
                        s += 1;
                    }
                    d => {
                        if s == 1 {
                            x = x * 10 + ((d.to_digit(10).unwrap()) as i32);
                        } else {
                            y = y * 10 + d.to_digit(10).unwrap() as i32;
                        }
                        s += 2;
                    }
                },
                2 | 5 => {
                    if s == 2 {
                        x = x * 10 + i.to_digit(10).unwrap() as i32;
                    } else {
                        y = y * 10 + i.to_digit(10).unwrap() as i32;
                    }
                    s += 1;
                }
                3 => match i {
                    '+' => {
                        s = 4;
                    }
                    d => {
                        x = x * 10 + d.to_digit(10).unwrap() as i32;
                    }
                },
                6 => match i {
                    'i' => s = 7,
                    d => {
                        y = y * 10 + d.to_digit(10).unwrap() as i32;
                    }
                },
                d => {
                    continue;
                }
            }
        }

        ComplexNumber {
            x: sx * x,
            y: sy * y,
        }
    }

    fn to_string(self: &Self) -> String {
        format!("{}+{}i", self.x, self.y)
    }
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let n1 = ComplexNumber::from(&num1);
        let n2 = ComplexNumber::from(&num2);
        ComplexNumber {x: n1.x * n2.x - n1.y * n2.y, y: n1.x * n2.y + n1.y * n2.x}.to_string()
    }
}

fn main() {
    println!("Hello, world!");
}
