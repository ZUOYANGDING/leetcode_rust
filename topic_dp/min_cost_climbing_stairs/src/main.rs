/**
 * Leetcode 746
 */
struct Solution;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        if cost.len() <= 1 {
            return 0;
        }
        let mut dp = vec![];
        dp.push(0);
        dp.push(0);
        for idx in 2..=cost.len() {
            dp.push(std::cmp::min(
                dp[idx - 1] + cost[idx - 1],
                dp[idx - 2] + cost[idx - 2],
            ));
        }
        dp[cost.len()]
    }
}
fn main() {
    assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    assert_eq!(
        6,
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
    assert_eq!(1, Solution::min_cost_climbing_stairs(vec![1, 100]));
}
