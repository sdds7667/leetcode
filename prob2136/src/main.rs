struct Solution {}

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut pts: Vec<_> = plant_time.into_iter().zip(grow_time.into_iter()).collect();
        pts.sort_unstable_by_key(|x| x.1);
        let mut ct = 0;
        for (pt, gt) in pts {
            if ct <= gt {
                ct = pt + gt;
            } else {
                ct = pt + ct;
            }
        }
        ct
    }
}

fn main() {
    println!("Hello, world!");
}
