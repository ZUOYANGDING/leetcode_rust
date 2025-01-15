/**
 * Leetcode 78
 */
struct Solution;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cur_set = vec![];
        let mut ret = vec![];
        ret.push(cur_set.clone());
        Self::trace_back(&nums, 0, &mut cur_set, &mut ret);
        ret
    }

    fn trace_back(
        nums: &Vec<i32>,
        cur_idx: usize,
        cur_set: &mut Vec<i32>,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if cur_idx > nums.len() {
            ret.push(cur_set.clone());
            return;
        }

        for idx in cur_idx..nums.len() {
            cur_set.push(nums[idx]);
            ret.push(cur_set.clone());
            Self::trace_back(nums, idx + 1, cur_set, ret);
            cur_set.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = Vec::from([1, 2, 3]);
        println!("{:?}", Solution::subsets(nums));
    }
}
