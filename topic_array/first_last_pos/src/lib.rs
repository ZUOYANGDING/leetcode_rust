use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left_bound = Self::left_bound(&nums, target);
        let right_bound = Self::right_bound(&nums, target);
        if left_bound == -2 || right_bound == -2 {
            return vec![-1, -1];
        } else if right_bound - left_bound > 1 {
            return vec![left_bound + 1, right_bound - 1];
        } else {
            return vec![-1, -1];
        }
    }

    // left bound exclude the index of the target
    fn left_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 - 1;
        let mut left_bound = -2; //in case the target at index 0
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {
                right = mid - 1;
                left_bound = right;
            } else {
                left = mid + 1;
            }
        }
        return left_bound;
    }

    // right bound exclude the index of the target
    fn right_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 - 1;
        let mut right_bound = -2; //in case the target at index len()-1
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] <= target {
                left = mid + 1;
                right_bound = left;
            } else {
                right = mid - 1;
            }
        }
        return right_bound;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let mut target = 8;
        assert_eq!(vec![3, 4], Solution::search_range(nums.clone(), target));
        target = 6;
        assert_eq!(vec![-1, -1], Solution::search_range(nums.clone(), target));
        let nums = vec![];
        target = 0;
        assert_eq!(vec![-1, -1], Solution::search_range(nums.clone(), target));
        let nums = vec![1];
        let mut target = 1;
        assert_eq!(vec![0, 0], Solution::search_range(nums.clone(), target));
    }
}
