/**
 * Leetcode 1049
 * Idea:
 *  This questions can be regarded as set sign before each number of the stone array, and get the minimum sum.
 *  Therefore, the question can be transformed into question: how to seperate the array into 2 subset, then make the sum of them as close as possible.
 *  Then this question is similar to the question Leetcode 416, and it can be applied by 01 knapsack
 */
struct Solution;
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        if stones.len() == 1 {
            return stones[0];
        }

        let sum_weight = stones.iter().sum::<i32>();

        let mut dp = vec![0; (sum_weight / 2 + 1) as usize];
        for i in 0..stones.len() {
            for j in (stones[i] as usize..=(sum_weight / 2) as usize).rev() {
                dp[j] = std::cmp::max(dp[j], dp[j - stones[i] as usize] + stones[i]);
            }
        }

        sum_weight - 2 * dp[(sum_weight / 2) as usize]
    }
}
fn main() {
    assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
}
