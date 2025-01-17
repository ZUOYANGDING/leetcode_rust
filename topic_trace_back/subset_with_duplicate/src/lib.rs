struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut cur_set = vec![];
        let mut ret = vec![];
        ret.push(cur_set.clone());
        Self::dfs(&nums, 0, &mut cur_set, &mut ret);
        ret
    }

    fn dfs(nums: &Vec<i32>, cur_idx: usize, cur_set: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if cur_idx > nums.len() {
            ret.push(cur_set.clone());
            return;
        }
        let mut idx = cur_idx;
        while idx < nums.len() {
            let cur_num = nums[idx];
            cur_set.push(cur_num);
            ret.push(cur_set.clone());
            Self::dfs(nums, idx + 1, cur_set, ret);
            cur_set.pop();
            while idx < nums.len() && nums[idx] == cur_num {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let nums = Vec::from([1, 2, 2]);
        println!("{:?}", Solution::subsets_with_dup(nums));
    }
}
