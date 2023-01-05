struct Solution {}

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut sums = Vec::new();
        let mut c: i64 = 0;
        for i in nums {
            c += i as i64;
            sums.push(c);
        }

        let mut res = Vec::new();

        for i in queries {
            res.push(Solution::binary_search(&sums, i as i64));
        }

        return res;
    }

    fn binary_search(sums: &Vec<i64>, target: i64) -> i32 {
        println!("{:?}", sums);
        let mut l = 0usize;
        let mut r = sums.len() - 1;
        while l <= r {
            let m = (l+r)/2;
            if sums[m] == target {
                return m as i32 + 1;
            }
            if sums[m] > target {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        r as i32 + 1
    }
}

fn main() {
    println!("{:?}", Solution::answer_queries(vec![4,5,2,1], vec![10]));
    println!("Hello, world!");
}
