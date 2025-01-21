/**
 * Leetcode 376
 */
struct Solution;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        // if length greater or equal to 2
        // append a num=nums[0] before the nums and a num=nums[1] after nums
        // that make sure the max length return will be 1
        let mut pre_diff = 0;
        let mut sur_diff = 0;
        let mut ret = 1;
        let mut idx = 0;
        while idx < nums.len() - 1 {
            // skip a number in monotonically increasing or decreasing trends
            // or a number in a flat trends
            // especially for number in a flat trends, keep the last one
            sur_diff = nums[idx + 1] - nums[idx];
            if (pre_diff >= 0 && sur_diff < 0) || (pre_diff <= 0 && sur_diff > 0) {
                ret += 1;
                // only update the pre_dif when there exsiting a trends change
                // in case the trends like increse->flatten->increase
                pre_diff = sur_diff;
            }
            idx += 1;
        }
        ret
    }
}
fn main() {
    let nums = vec![1, 7, 4, 9, 2, 5];
    assert_eq!(Solution::wiggle_max_length(nums), 6);
    let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    assert_eq!(Solution::wiggle_max_length(nums), 7);
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::wiggle_max_length(nums), 2);
}
