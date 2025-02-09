/**
 * Leetcode 452
 * Idea:
 *  The question can be regarded as: There are several segments overlap each other or not on number line.
 *  Then put dots on those segments, to let each segment contains 1 dot at least.
 * Solve:
 *  Sort these segments by the left bound in increasing order
 *  Then need to find a way to define "as much overlap as possible into a group", which means how to define the left and right bound of each group.
 *  There are 2 tipical condition after sorting:
 *      1. [1,6], [2,8], [7,12], [10,16]: [1,6] and [2,8] can be in a group 1, since from [7,12] the left bound already greater than min right bound of the group 1.
 *      However, in this case, the right bound of group 1 is occasionally increasing after sorting, but not always like this.
 *      2. [1,10], [3,9], [4,11], [6,7], [6,9], [9,12]: For the left bound part, same logic as above example (when a new segment's left bound grater than min right bound of prev group),
 *      but we need to update the min right bound of the group 1. The reason is after sorting the right bound of each segment are not monotone non-increasing.
 *      Therefore, the logic to update the min right bound is always choose the min of each segement as the right bound of the group when construct a group.
 */
struct Solution;
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1;
        }
        points.sort_by(|a, b| {
            if a[0] == b[0] {
                return a[1].cmp(&b[1]);
            }
            a[0].cmp(&b[0])
        });
        let mut min_right = points[0][1];
        let mut ret = 1;
        for idx in 0..points.len() {
            if points[idx][0] > min_right {
                ret += 1;
                min_right = points[idx][1];
            }
            if points[idx][1] < min_right {
                min_right = points[idx][1];
            }
        }
        ret
    }
}
fn main() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );
    assert_eq!(
        Solution::find_min_arrow_shots(vec![
            vec![9, 12],
            vec![1, 10],
            vec![4, 11],
            vec![8, 12],
            vec![3, 9],
            vec![6, 9],
            vec![6, 7]
        ]),
        2
    );
}
