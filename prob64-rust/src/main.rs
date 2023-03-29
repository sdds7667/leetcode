struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        for i in 1..m {
            grid[n - 1][i] += grid[n - 1][i - 1];
        }

        for i in 1..n {
            grid[i][m - 1] += grid[i - 1][m - 1];
        }

        for i in 1..n {
            for j in 1..m {
                grid[i][j] += grid[i-1][j].min(grid[i][j - 1]);
            }
        }

        return grid[n-1][m-1];
    }
}

fn main() {
    println!("Hello, world!");
}
