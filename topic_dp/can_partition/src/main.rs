use std::vec;

/**
 * Leetcode 416
 * Idea: To partition a number into 2 subset with same sum:
 *          1. if total sum is a even number, if it is not, just false
 *          2. We need to find a way to pick k numbers from num array that sum of them is total_sum/2
 *          3. Then the question transform to a 01 knapsack problem that a knapsack with max weight as total_sum/2,
 *          the array is the item array, each item with a weight nums[i]. The target is to find if there exist a way
 *          to pick some of items to max the weight of knapsack(max means equal to total_sum/2)
 *          4. Apply the algorithm in standard 01 knapsack
 */
struct Solution;
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return false;
        }

        let sum = nums.iter().sum::<i32>();
        if sum % 2 != 0 {
            return false;
        }

        let sum = sum as usize;
        let mut dp = vec![0; sum / 2 + 1];
        for i in 0..nums.len() {
            for j in (nums[i] as usize..=sum / 2).rev() {
                dp[j] = std::cmp::max(dp[j], dp[j - nums[i] as usize] + nums[i]);
                if dp[j] == (sum / 2) as i32 {
                    return true;
                }
            }
        }
        false
    }
}
fn main() {
    assert_eq!(Solution::can_partition(vec![1,5,11,5]), true);
}
