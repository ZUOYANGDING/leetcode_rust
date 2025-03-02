/**
 * Leetcode 518
 *
 * Idea:
 *      Similar Idea to full knapsack problem
 *      Define dp[i][j] as record coins' combinations, when select coin 0..i with amount as j
 *      To get the number of combinations:
 *          1) do not choose coin i, the number of combination is dp[i-1][j];
 *          2) choose the coin i, the number of combination is dp[i][j-coins[i]];
 *      Therefore the total combination:
 *          dp[i][j] = dp[i-1][j] + dp[i][j-coins[i]], when j>=coins[i];
 *          dp[i][j] = dp[i-1][j], when j<coins[i];
 *      Init:
 *          i==0, means use coin 0 to fill up the amount. Therefore, dp[0][j]=1 when j%coins[0]==0;
 *          j==0, means use coins to fill up amount 0. Therefore, dp[i][0] = 1, which means do not choose the coin i.
 */

struct Solution;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; (amount + 1) as usize]; coins.len()];
        for idx in coins[0] as usize..=amount as usize {
            if idx % coins[0] as usize == 0 {
                dp[0][idx] = 1;
            }
        }
        for idx in 0..coins.len() {
            dp[idx][0] = 1;
        }
        for i in 1..coins.len() {
            for j in 1..=amount as usize {
                if j < coins[i] as usize {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - coins[i] as usize];
                }
            }
            // println!("{:?}", dp);
        }
        dp[coins.len() - 1][amount as usize]
    }
}
fn main() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4)
}
