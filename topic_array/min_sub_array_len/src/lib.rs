use std::cmp::{self, Ordering};

struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let length = nums.len() as i32;
        let mut p1 = 0;
        let mut p2 = 0;
        let mut ret = length + 1;
        let mut sum = nums[p1 as usize];
        while p2 < length {
            if (p1 != p2) {
                sum += nums[p2 as usize];
            }
            while sum >= target && p1 <= p2 {
                ret = cmp::min(ret, p2 - p1 + 1);
                sum -= nums[p1 as usize];
                p1 += 1;
            }
            p2 += 1;
        }
        if ret > length {
            return 0;
        } else {
            return ret;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let result = Solution::min_sub_array_len(7, nums);
        assert_eq!(result, 2);
    }
}
