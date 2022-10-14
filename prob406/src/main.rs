struct Solution {}

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_unstable_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });
        let mut queue = Vec::new();

        for i in people.iter() {
            queue.insert(i[1] as usize, i.clone());
        }

        queue
    }
}

fn main() {
    println!("Hello, world!");
}
