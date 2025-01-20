use std::collections::HashSet;

/**
 * Leetcode 46, 47
 */
struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut cur_set = vec![];
        let mut data_used = vec![false; nums.len()];
        Self::dfs_permute(&nums, &mut data_used, &mut cur_set, &mut ret);
        ret
    }

    fn dfs_permute(
        nums: &Vec<i32>,
        data_used: &mut Vec<bool>,
        cur_set: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if cur_set.len() == nums.len() {
            ret.push(cur_set.clone());
            return;
        }
        for idx in 0..nums.len() {
            if !data_used[idx] {
                data_used[idx] = true;
                cur_set.push(nums[idx]);
                Self::dfs_permute(nums, data_used, cur_set, ret);
                data_used[idx] = false;
                cur_set.pop();
            }
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut cur_set = vec![];
        let mut data_used = vec![false; nums.len()];
        Self::dfs_permute_unique(&nums, &mut data_used, &mut cur_set, &mut ret);
        ret
    }

    fn dfs_permute_unique(
        nums: &Vec<i32>,
        data_used: &mut Vec<bool>,
        cur_set: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if cur_set.len() == nums.len() {
            ret.push(cur_set.clone());
            return;
        }

        let mut idx = 0;
        let mut data_used_in_current_layer = HashSet::new();
        while idx < nums.len() {
            if !data_used[idx] && data_used_in_current_layer.insert(nums[idx]) {
                let cur_num = nums[idx];
                cur_set.push(cur_num);
                data_used[idx] = true;
                Self::dfs_permute_unique(nums, data_used, cur_set, ret);
                data_used[idx] = false;
                cur_set.pop();
            } else {
                idx += 1;
            }
        }
    }
}
fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", Solution::permute(nums));
    let nums = vec![3, 3, 0, 3];
    println!("{:?}", Solution::permute_unique(nums));
}
