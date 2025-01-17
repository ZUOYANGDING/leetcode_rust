use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn find_subsequences(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cur_set = vec![];
        let mut ret = vec![];
        Self::dfs(&nums, 0, &mut cur_set, &mut ret);
        ret
    }

    fn dfs(nums: &Vec<i32>, cur_idx: usize, cur_set: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if cur_idx > nums.len() {
            if cur_set.len() > 1 {
                ret.push(cur_set.clone());
            }
            return;
        }

        let mut idx = cur_idx;
        let mut num_used = HashSet::new();
        while idx < nums.len() {
            let cur_num = nums[idx];
            if !num_used.insert(cur_num) {
                idx += 1;
                continue;
            }
            if !cur_set.is_empty() {
                let size = cur_set.len();
                if cur_set[size - 1] <= cur_num {
                    cur_set.push(cur_num);
                    ret.push(cur_set.clone());
                } else {
                    idx += 1;
                    continue;
                }
            } else {
                cur_set.push(cur_num);
            }
            Self::dfs(nums, idx + 1, cur_set, ret);
            cur_set.pop();
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = Vec::from([4, 6, 7, 7]);
        println!("{:?}", Solution::find_subsequences(nums));
    }
}
