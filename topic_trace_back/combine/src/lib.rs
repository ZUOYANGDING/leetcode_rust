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
        // prune the no necessary iteration
        // cur indicate the number will start to pick
        // to prune, need to find the end of the iteration (possible useful number)
        // k - candidate.len() indicate how many numbers still need to add,
        // and they will be added during the continuous recursion, after pick the "num" at this layer ("num" like a start)
        // therefore, n - (k-candidate.len()) + 1 indicate the number which should be stop the iteration
        // For example: we have (n, k, cur) as (4,3,0), which means we at layer 0
        //              the iteration for this layer will be ended at 4-(3-0)+1 = 2
        //              which means 2 is the largest end to start recursion like [2,3,4]
        //              where [3,4] will be added in recursion start as picking 2
        for num in cur..=(n - (k - (candidate.len() as i32)) + 1) {
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
