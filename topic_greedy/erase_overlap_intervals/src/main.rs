/**
 * Leetcode 435
 * Idea: 2 ways:
 *      1. Sort intervals by left bound in non-decreasing order, then remove the overlap ones.
 *      So the key point is how to decide which interval need to remove when overlapping.
 *      To make the make the removal minimum, remove the larger interval, which means remove the interval with larger right bound
 *      2. Sort intervals by right bound in non-decreasing order, then count non-overlapping intervals, then use total deduct the non-overlapping intervals
 *      So the key point is how to decide which intervals is non-overlapping. When the prev interval right bound is less than the current left bound,
 *      it is not overlapping.
 */
struct Solution;
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 1 {
            return 0;
        }
        intervals.sort_unstable();
        let mut ret = 0;
        let mut right_bound = intervals[0][1];
        for idx in 1..intervals.len() {
            if intervals[idx][0] < right_bound {
                if intervals[idx][1] < right_bound {
                    right_bound = intervals[idx][1];
                }
                ret += 1;
            } else {
                right_bound = intervals[idx][1];
            }
        }
        ret
    }
}
fn main() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
}
