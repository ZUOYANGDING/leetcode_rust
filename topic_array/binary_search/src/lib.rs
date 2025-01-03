struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return Self::helper(nums.clone(), 0, (nums.len() - 1) as i32, target);
    }
    fn helper(nums: Vec<i32>, left: i32, right: i32, target: i32) -> i32 {
        let mid = (left + right) / 2;
        if left > right {
            return -1;
        } else if *nums.get(mid as usize).unwrap() == target {
            return mid as i32;
        } else if *nums.get(mid as usize).unwrap() > target {
            return Self::helper(nums, left, mid - 1, target);
        } else {
            return Self::helper(nums, mid + 1, right, target);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search(nums.clone(), 9), 4);
        assert_eq!(Solution::search(nums, 2), -1);
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 13), -1);
    }
}
