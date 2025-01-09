/**
 * Leetcode 40
 */

struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut queue = vec![];
        let mut ret = vec![];
        Self::trade_back(target, &candidates, 0, 0, &mut queue, &mut ret);
        ret
    }

    fn trade_back(
        target: i32,
        candidates: &Vec<i32>,
        cur_idx: usize,
        cur_sum: i32,
        queue: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if cur_sum == target {
            ret.push(queue.clone());
            return;
        }

        let mut idx = cur_idx;
        while idx < candidates.len() {
            if cur_sum + candidates[idx] > target {
                // sorted candidates
                // rest number in candidates cannot be smaller than current
                return;
            }
            queue.push(candidates[idx]);
            Self::trade_back(
                target,
                candidates,
                idx + 1,
                cur_sum + candidates[idx],
                queue,
                ret,
            );
            queue.pop();
            let mut cur_num = candidates[idx];
            // filter duplicate
            // For a possible solution, number on each position (means layer of the search tree)
            // cannot be picked as same number.
            // For a example, if a candidate [1,2,2] and target 3
            //  possible solution is [1,2], and the the number on second position (second layer in recursion)
            //  which is 2, but can only pick the 2nd in candidate or 3rd in candidate
            //  cannot pick both. Otherwise, there are duplicated answer [1,2] and [1,2]
            // Which means in our search tree, number can duplicate (caused by duplicate number in candidates)
            // in the path to the leaf, but cannot be duplicate at the same layer
            while idx + 1 < candidates.len() && cur_num == candidates[idx + 1] {
                idx += 1;
                cur_num = candidates[idx];
            }
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        println!("{:?}", Solution::combination_sum2(candidates, 8));
    }
}
