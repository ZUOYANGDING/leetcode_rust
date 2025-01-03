struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let length = nums.len() as i32;
        let mut p1 = 0;
        let mut p2 = 0;
        while p2 < length {
            if nums[p2 as usize] != val {
                let temp = nums[p1 as usize];
                nums[p1 as usize] = nums[p2 as usize];
                nums[p2 as usize] = temp;
                p1 += 1;
                p2 += 1;
            } else {
                p2 += 1;
            }
        }
        return p1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let ret = Solution::remove_element(&mut nums, 2);
        assert_eq!(ret, 5);
    }
}
