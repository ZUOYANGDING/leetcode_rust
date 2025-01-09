/**
 * Leetcode 39
 */

struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut queue = vec![];
        let mut ret = vec![];

        Self::trace_back(target, &candidates, 0, 0, &mut queue, &mut ret);
        ret
    }

    fn trace_back(
        target: i32,
        candidates: &Vec<i32>,
        cur_sum: i32,
        cur_idx: usize,
        queue: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if cur_sum == target {
            ret.push(queue.clone());
            return;
        }
        for idx in cur_idx..candidates.len() {
            queue.push(candidates[idx]);
            if cur_sum + candidates[idx] > target {
                queue.pop();
                continue;
            }
            Self::trace_back(
                target,
                candidates,
                cur_sum + candidates[idx],
                idx,
                queue,
                ret,
            );
            queue.pop();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let candidates = vec![2, 3, 6, 7];
        println!("{:?}", Solution::combination_sum(candidates, 7));
    }
}
