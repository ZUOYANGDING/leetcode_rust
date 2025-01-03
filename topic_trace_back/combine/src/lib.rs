use std::collections::HashSet;

/// https://leetcode.cn/problems/combinations/description/
struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut candidate: Vec<i32> = Vec::new();
        Self::trace_back(n, k, 1, &mut candidate, &mut ret);
        ret
    }

    pub fn trace_back(
        n: i32,
        k: i32,
        cur: i32,
        candidate: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if candidate.len() as i32 == k {
            result.push(candidate.clone()); // already contains k elements
            return;
        }
        for num in cur..=n {
            candidate.push(num); // pick this number
            Self::trace_back(n, k, num + 1, candidate, result); //
            candidate.pop(); // unpick this number
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::combine(4, 2));
    }
}
