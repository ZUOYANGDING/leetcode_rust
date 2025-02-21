/**
 * Leetcode 343
 * Idea: For a number i, define dp[i] as the largest result. But need to remeber dp[i] is the result that seperate i as at least 2 numbers
 *       To get dp[i], we need to choose the largest result among j*(i-j), j*dp[i-j], where j<i.
 *       j*(i-j) means we seperate the number i as 2 different number
 *       j*dp[i-j] means we solid the j, then for every number from j..=i, we seperate them into at least 2 numbers.
 *       The reason we do not use dp[j]*dp[i-j] is, that for dp[j] it is a result of seperate j as at least 2 numbers; if apply dp[j]*dp[i-j],
 *       we will miss all conditions that regard j as a whole number to get result.
 *       
 *       Since j*(i-j) is symmetric for 1..i/2 and i/2..i, and dp[j] and dp[i-j] are both recorded already when calculate dp[i],
 *       the inner loop just iter from 1..i/2, and each iter choose the max among j*(i-j), dp[j]*(i-j), j*dp[i-j], and cur_max
 */
struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 1;
        for idx in 3..=n {
            let mut cur_max = dp[idx];
            for deduct in 1..=idx / 2 {
                let inner_max = std::cmp::max(
                    (deduct * (idx - deduct)) as i32,
                    std::cmp::max(
                        (deduct as i32) * dp[idx - deduct],
                        dp[deduct] * (idx - deduct) as i32,
                    ),
                );
                cur_max = std::cmp::max(cur_max, inner_max);
            }
            dp[idx] = cur_max;
        }
        dp[n]
    }
}
fn main() {
    assert_eq!(Solution::integer_break(8), 18);
}
