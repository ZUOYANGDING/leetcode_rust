use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut set: HashSet<Vec<i32>> = HashSet::new();

        for idx_i in 0..nums.len() {
            // prune
            if nums[idx_i] > target && nums[idx_i] >= 0 {
                break;
            }
            for idx_j in idx_i + 1..nums.len() {
                // prune
                if nums[idx_j] + nums[idx_i] > target && nums[idx_j] >= 0 {
                    break;
                }
                let mut left = idx_j + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum = nums[idx_i] as i64
                        + nums[idx_j] as i64
                        + nums[left] as i64
                        + nums[right] as i64;
                    if sum < target as i64 {
                        left += 1;
                    } else if sum > target as i64 {
                        right -= 1;
                    } else {
                        // filter the duplicate
                        if !set.contains(&vec![nums[idx_i], nums[idx_j], nums[left], nums[right]]) {
                            set.insert(vec![nums[idx_i], nums[idx_j], nums[left], nums[right]]);
                        }
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
        set.into_iter().map(|item| item).collect::<Vec<Vec<i32>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
        // println!("{:?}", nums.iter().sum::<i64>());
        println!("{:?}", Solution::four_sum(nums, -294967296));
    }
}
