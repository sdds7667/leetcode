
struct Solution {}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();
        let h = grid.len();
        let w = grid[0].len();
        let wi = grid[0].len() as i32;
        'rows: for i in 0..w {
            let mut x = i;
            let mut y = 0;
            while y < h {
                let nx = (x as i32) + grid[y][x] ;
                if nx == -1 || nx == wi{
                    res.push(-1);
                    continue 'rows;
                }
                let nx = nx as usize;
                if grid[y][nx] != grid[y][x] {
                    res.push(-1);
                    continue 'rows;
                }
                y += 1;
                x = nx;
            }
            res.push(x as i32);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
