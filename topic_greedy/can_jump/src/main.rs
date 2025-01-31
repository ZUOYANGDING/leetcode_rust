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

    // Leetcode 45
    // This Question still need to consider what is the farest idx each jump can cover
    // As greedy, for each jump we want to jump as far as we can; for global we want jump as less as we can
    // That means for each jump (farest) we want to know:
    //  1. if we can approach the end of the array
    //  2. if we need this jump to approach the next jump start idx
    // The reason for the "2" is e.g. [2,1,3,3,3,5,2,1,1] for the 3 concat "3", we need to know which one we should pick as the jump start position
    // Therefore, for each possible jump, we can maintain a small check window, to see if its possible to approach the end by jump again inside this
    // window (use this jump's covering segment as a start of next jump). If not means, we **might** need to use out this jump's covering.
    // The reason for "might" is, that we only pick the slot with farest jump covering as next jump start. e.g. in example, if array[6]=1 which cannot approach
    // end, we choose array[5] as next jump start, and `prev_far = 7`
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        // previous farest index can approach
        let mut prev_far = 0;
        // current farest index can approach
        let mut cur_far = 0;
        // steps
        let mut step = 0;
        for idx in 0..nums.len() {
            cur_far = std::cmp::max((nums[idx] as usize) + idx, cur_far);
            // small check window
            // until the idx approach the pre_far step+1, means must jump from the prev_far's position
            // or end of array
            if idx == prev_far {
                // update the farest index can approach
                step += 1;
                prev_far = cur_far;
                if prev_far >= nums.len() - 1 {
                    return step;
                }
            }
        }
        step
    }
}
fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
}
