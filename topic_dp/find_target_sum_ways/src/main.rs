/**
 * Leetcode 494
 * Idea: Suppose the sum of number with '+' sign is a, and sum of number with '-' is b, and suppose the sum of all is s
 *       Then we have a+b=s, and a-b=target. Then we get a = (s+target)/2, b = (s-target)/2
 *       For target>0, a>b, and for target<0, a<b.
 *       For all above, the question can be trasformed to:
 *          there is a knapsack with most weight as (s-target.abs())/2, and items with weight as array nums,
 *          try to find the number of combination of picking to fillup the knapsack
 *       The reason to apply s-target.abs() is to opitmize the complexity
 *       
 *       To solve the transformed questions:
 *          1. If s<target.abs() or s-target.abs() is a odd number, which means there is no answer
 *          2. Use dp[i][j] to record the number of combination of picking from item 0..i with knapsack weight j
 *          3. To decide the dp[i][j]:
 *              1) do not choose item i: dp[i-1][j]
 *              2) choose item i: dp[i-1][j-nums[i]]
 *             Therefore, the total combination is dp[i][j] = dp[i-1][j] + dp[i-1][j-nums[i]]
 *          4. Init:
 *              1) when i==0: pick item 0 for all possible weight 0..j
 *              2) when j==0: pick item 0..i for weight j:
 *                      i) do not choose the item i when item i's weight greater than 0
 *                      ii) choose item i when item i's weight is 0
 */
struct Solution;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if (sum - target.abs()) % 2 != 0 || sum < target.abs() {
            return 0;
        }

        let sum_target = (sum - target.abs()) / 2;
        let mut dp = vec![vec![0; (sum_target + 1) as usize]; nums.len()];

        let item_0_weight = nums[0];
        if item_0_weight <= sum_target {
            dp[0][item_0_weight as usize] = 1;
        }
        let mut num_zeros = 0;
        for idx in 0..nums.len() {
            if nums[idx] == 0 {
                num_zeros += 1;
            }
            dp[idx][0] = 1 << num_zeros;
        }

        for i in 1..nums.len() {
            for j in 1..=sum_target as usize {
                if j < nums[i] as usize {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - nums[i] as usize];
                }
            }
            // println!("{:?}", dp);
        }
        dp[nums.len() - 1][sum_target as usize]
    }
}
fn main() {
    assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}
