/**
 * Leetcode 377
 *
 * Idea:
 *      This questions is a permutation problem with duplicate number choosing.
 *      In each recursion, pick a num from nums make the sum less or equal to target until the sum equal to target,
 *      the combinatin of picking will be one of the possible answer.
 *      However, we can use an array to record the number of combinations for each recursion(each sum from 0..target), to decrease the complexity
 *      
 *      Define dp[j] record the total number combination when target is j
 *      For each target j, the combination will be (dp[j-nums[0]]+..+dp[j-nums[i]]), which means the sum of possible combination when choose from nums 0..i
 */

struct Solution;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;

        for j in 1..=target as usize {
            for i in 0..nums.len() {
                if j >= nums[i] as usize {
                    dp[j] += dp[j - nums[i] as usize];
                }
            }
        }
        dp[target as usize]
    }
}
fn main() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
}
