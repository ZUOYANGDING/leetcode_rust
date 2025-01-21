/**
 * Leetcode 455 and 2410
 */
use std::cmp;
struct Solution;
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        if s.len() < 1 {
            return 0;
        }
        // sort g and s in descending
        g.sort_by(|a, b| b.cmp(a));
        s.sort_by(|a, b| b.cmp(a));
        // find the idx of Min(g(idx)) <= Max(s), which is first possible fit
        let mut idx_g = 0;
        while idx_g < g.len() {
            if s[0] >= g[idx_g] {
                break;
            }
            idx_g += 1;
        }
        let mut ret = 0;
        let mut idx_s = 0;
        while idx_s < s.len() && idx_g < g.len() {
            if g[idx_g] <= s[idx_s] {
                ret += 1;
                idx_s += 1;
                idx_g += 1;
            } else {
                // cannot return here
                // number in g might repeated
                idx_g += 1;
            }
        }
        return ret;
    }
}

fn main() {
    let g = vec![10, 9, 8, 7, 10, 9, 8, 7];
    let s = vec![10, 9, 8, 7];
    assert_eq!(Solution::find_content_children(g, s), 4);
}
