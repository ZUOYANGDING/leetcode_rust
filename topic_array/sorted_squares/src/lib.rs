struct Solution;
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len() as i32;
        let mut ret = vec![0; length as usize];
        let mut p1 = 0;
        let mut p2 = length - 1;
        let mut p3 = length;
        while p3 > 0 {
            if nums[p1 as usize] * nums[p1 as usize] > nums[p2 as usize] * nums[p2 as usize] {
                ret[p3 as usize - 1] = nums[p1 as usize] * nums[p1 as usize];
                p1 += 1;
            } else {
                ret[p3 as usize - 1] = nums[p2 as usize] * nums[p2 as usize];
                p2 -= 1;
            }
            p3 -= 1;
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-5, -3, -2, -1];

        let nums_1 = vec![1, 4, 9, 25];
        let result = Solution::sorted_squares(nums);
        for (n, r) in result.iter().zip(nums_1) {
            assert_eq!(*n, r);
        }
    }
}
