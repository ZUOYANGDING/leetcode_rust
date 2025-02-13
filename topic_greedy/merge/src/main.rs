/**
 * Leetcode 56
 *
 * Idea: Similar to 763/435, sort by left bound, then try to update the left and right bound to merge overlapping segment
 */

struct Solution;
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1 {
            return intervals;
        }
        intervals.sort_unstable();
        let mut ret = vec![];
        let mut left_bound = intervals[0][0];
        let mut right_bound = intervals[0][1];
        for idx in 1..intervals.len() {
            if intervals[idx][0] > right_bound {
                ret.push(vec![left_bound, right_bound]);
                left_bound = intervals[idx][0];
                right_bound = intervals[idx][1];
            } else if intervals[idx][1] > right_bound {
                right_bound = intervals[idx][1];
            }
        }
        ret.push(vec![left_bound, right_bound]);
        ret
    }
}

fn main() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
}
