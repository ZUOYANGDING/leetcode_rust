struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums[0] > target {
            return 0_i32;
        }
        if nums[nums.len() - 1] < target {
            return nums.len() as i32;
        }
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        while left <= right {
            let mid: i32 = (left + right) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] >= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 3, 5, 6];
        let mut target = 5;
        assert_eq!(Solution::search_insert(nums.clone(), target), 2);
        target = 2;
        assert_eq!(Solution::search_insert(nums.clone(), target), 1);
        target = 7;
        assert_eq!(Solution::search_insert(nums.clone(), target), 4);
    }
}
