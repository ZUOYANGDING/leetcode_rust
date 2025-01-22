/**
 * Leetcode 53
 */
struct Solution;
impl Solution {
    // We need to do is, that whether include the negative sequence or not
    // The decision made by sum_of_negative_sequence + consecutive positive number > consecutive positive number itself
    pub fn max_sub_array_not_clean(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        let mut segment_sum = nums[0];
        let mut idx = 1;
        while idx < nums.len() {
            if nums[idx] >= 0 {
                // when number>0, 2 situations
                // 1. just a positive number after other positive number, add the number directly
                // 2. a positive number after a negative sequence. Need to decide include the negative sequence
                //    or just skip the negative sequence
                segment_sum = std::cmp::max(nums[idx], segment_sum + nums[idx]);
                if ret < segment_sum {
                    ret = segment_sum;
                }
                idx += 1;
            } else {
                // when number < 0, try to find the next non-negative number
                let mut idx_temp = idx;
                let mut temp_segment_sum = 0;
                // Deal with a situation that all number in array is negetive
                let mut max_in_current_seg = nums[idx];
                while idx_temp < nums.len() && nums[idx_temp] < 0 {
                    temp_segment_sum += nums[idx_temp];
                    if max_in_current_seg < nums[idx_temp] {
                        max_in_current_seg = nums[idx_temp];
                    }
                    idx_temp += 1;
                }
                if idx_temp == nums.len() {
                    // if reach the end, just break
                    if ret < max_in_current_seg {
                        ret = max_in_current_seg;
                    }
                    break;
                } else {
                    // if not, include the negative sequence temporarily
                    // and let the outside loop to decide to include the sequence or not
                    segment_sum += temp_segment_sum;
                    idx = idx_temp;
                }
            }
        }
        ret
    }

    // In the above version, logic is a little bit complicated
    // Suppose we have a slide window and sum of number inside it is 0
    // move the right side of the window to include the new number
    // If the new number make the sum<0, move the left side of the window to the next of the new number, until we found a non-negative number to included in
    // Because if we include the number and the next one is non-negative, sum_of_window+next_one < next_one
    // If the new number make the sum>0, we can include this new number and extend the right side of the window to next number
    // Because if we include the number (even it is a negative number) and the next one is non-negative, sum_of_window+next_one > next_one
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // to dealing with all elements in array are negative
        let mut ret = i32::MIN;
        let mut segment_sum = 0;
        for idx in 0..nums.len() {
            segment_sum += nums[idx];
            // Must be ordering like following to deal with all elements in array are negative
            ret = std::cmp::max(segment_sum, ret);
            segment_sum = std::cmp::max(segment_sum, 0);
        }
        ret
    }
}
fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{:?}", Solution::max_sub_array_not_clean(nums));
    let nums = vec![-2, -1];
    println!("{:?}", Solution::max_sub_array(nums));
}
