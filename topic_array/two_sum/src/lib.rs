use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (idx, num) in nums.iter().enumerate() {
            let _ = map.entry(target - num).or_insert(idx);
        }
        for (idx, num) in nums.into_iter().enumerate() {
            match map.get(&num) {
                Some(value) => {
                    if idx != *value {
                        return vec![idx as i32, *value as i32];
                    } else {
                        continue;
                    };
                }
                None => continue,
            }
        }
        vec![]
    }
}

mod test {
    use crate::Solution;

    #[test]
    fn test_case() {
        let nums = vec![3, 3];
        let target = 6;
        let solution = Solution::two_sum(nums, target);
        assert!(solution.contains(&0));
        assert!(solution.contains(&1));
    }
}
