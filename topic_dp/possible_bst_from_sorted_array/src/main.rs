/**
 * Leetcode 96
 * 
 * Idea: For a binary search built from a sorted array in increasing order, every element in array can be set as root
 *       Therefore, the total number of possilbe trees is sum of possible bst for each element set as root.
 *       Since it is a bst, which means left cannot be greater than right. Therefore, when set i as root (where 1<i<n)
 *       there will be i-1 nodes on left and n-i+1 nodes on right. The number of possible different bst for i set as root
 *       will be (number of possible bst for i-1 nodes) * (number of possible bst for n-i nodes).
 * 
 *       For the analysis above, define the dp array as dp[i] record the number of possible bst for i nodes.
 *       we can get dp[i] = bst set 1 as root + bst set 2 as root + .... + bst set i as root
 *                        = dp[0]*dp[i-1] + dp[1]*dp[i-2] +...+dp[i-1]*dp[0] 
 *       
 */
struct Solution;
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n+1];
        dp[0] = 1;
        for i in 1..=n {
            for j in 0..i {
                dp[i] += dp[j]*dp[i-j-1];
            }
        }
        dp[n] 
    }
}
fn main() {
    assert_eq!(Solution::num_trees(4), 14);
}
