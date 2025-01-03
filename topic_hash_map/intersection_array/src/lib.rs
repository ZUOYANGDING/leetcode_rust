use std::collections::{hash_set, HashSet};
struct Solution;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut hash_set = HashSet::new();
        nums1.into_iter().for_each(|n| {
            hash_set.insert(n);
        });
        let ret: Vec<i32> = hash_set
            .into_iter()
            .filter(|n| nums2.contains(&n))
            .collect();
        return ret;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        let result = Solution::intersection(nums1, nums2);
        println!("{:?}", result);
    }
}
