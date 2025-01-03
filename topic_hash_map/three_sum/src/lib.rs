struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        if nums[0] > 0 {
            return vec![];
        }
        let mut ret = Vec::new();
        let mut set = HashSet::new();
        for idx in 0..nums.len() - 2 {
            let mut left = idx + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[idx] + nums[left] + nums[right];
                if sum == 0 && !set.contains(&vec![nums[idx], nums[left], nums[right]]) {
                    ret.push(vec![nums[idx], nums[left], nums[right]]);
                    set.insert(vec![nums[idx], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if sum > 0 {
                    right -= 1
                } else {
                    left += 1;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![0, 0, 0];
        let ret = Solution::three_sum(v);
        println!("{:?}", ret);
    }
}
