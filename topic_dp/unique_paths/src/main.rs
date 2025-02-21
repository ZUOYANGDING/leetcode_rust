/**
 * Leetcode 62, 63
 */
struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; n as usize]; m as usize];
        for row in 1..m as usize {
            for col in 1..n as usize {
                dp[row as usize][col as usize] = dp[row - 1][col] + dp[row][col - 1];
            }
        }
        dp[(m - 1) as usize][(n - 1) as usize]
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let row_num = obstacle_grid.len();
        let col_num = obstacle_grid[0].len();
        let mut dp = vec![vec![0; col_num]; row_num];
        for row in 0..row_num {
            for col in 0..col_num {
                if obstacle_grid[row][col] == 1 {
                    dp[row][col] = 0;
                    continue;
                }
                if row == 0 && col == 0 {
                    dp[row][col] = 1;
                } else if row == 0 {
                    dp[row][col] = dp[row][col - 1];
                } else if col == 0 {
                    dp[row][col] = dp[row - 1][col];
                } else {
                    dp[row][col] = dp[row - 1][col] + dp[row][col - 1];
                }
            }
        }
        dp[row_num - 1][col_num - 1]
    }
}
fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(
        Solution::unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        2
    );
}
