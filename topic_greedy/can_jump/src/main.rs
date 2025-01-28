/**
 * Leetcode 55
 */
struct Solution;
impl Solution {
    // Consider at position i, if the i+nums[i] can exceed the nums.len()
    // Update the `cover` to indicate how far can jump until approach index i
    // and if at the index i nums[i]==0 and cover==i, but i is not the end of the array
    // that means cannot reach the end forever
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut cover = 0;
        if nums.len() == 1 {
            return true;
        }
        let mut idx = 0;
        while idx <= cover {
            cover = std::cmp::max(cover, idx + (nums[idx] as usize));
            if cover >= nums.len() - 1 {
                return true;
            }
            idx += 1;
        }
        false
    }
}
fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
}
