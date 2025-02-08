/**
 * Leetcode 406
 * Idea: The problem is to decide how to sort the input, since there are 2 factors: hight, how many people greater/equal to this hight
 *       Therefore, can only sort by one factor, than apply the other factor to do modification
 *          If sort by hight first from high to low first. After sorting, for element i (hi, ki), there is garantee that all elements j<i, hi<hj.
 *          Therefore, just need to put the i to index ki to make sure there are k elements where hk>hi
 *          If sort by how many people greater/equal to this height from low to hight first. After sorting, for element i (hi, ki), there is garantee that
 *          all elements j<i, kj<ki. However, there is no garantee that there are ki elements before i, which means after this sorting, we do not get any
 *          improve.
 *       Therefore, sort by the hight first.
 */
use std::collections::VecDeque;
struct Solution;
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                return a[1].cmp(&b[1]);
            }
            b[0].cmp(&a[0])
        });
        // After sorting, the hight is decreasing
        // If there is (7,0), (5,0); (7,0) will be push into queue first, then (5,0) will be inserted and take the position 0
        // In this way, no need to adjust the order of (7,0), (5,0) again
        let mut queue = VecDeque::new();
        for p in people {
            queue.insert(p[1] as usize, p);
        }
        let ret: Vec<Vec<i32>> = queue.into_iter().collect();
        ret
    }
}
fn main() {
    let ret = Solution::reconstruct_queue(vec![
        vec![7, 0],
        vec![4, 4],
        vec![7, 1],
        vec![5, 0],
        vec![6, 1],
        vec![5, 2],
    ]);
    assert_eq!(
        vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1]
        ],
        ret
    );
}
