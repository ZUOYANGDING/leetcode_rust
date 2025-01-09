/**
 * Leetcode 216
 */
struct Solution;
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut candidate = Vec::new();
        let mut ret = Vec::new();
        Self::combination(k, n, &mut candidate, 0, 1, &mut ret);
        ret
    }

    fn combination(
        k: i32,
        n: i32,
        candidate: &mut Vec<i32>,
        cur_sum: i32,
        cur_num: i32,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if (candidate.len() as i32) == k {
            if cur_num == n {
                ret.push(candidate.clone());
            }
            return;
        }

        // prune for iteration as "combination"
        for num in cur_num..=(9 - (k - (candidate.len() as i32)) + 1) {
            candidate.push(num);
            // prune for recursion
            if cur_sum + num > n {
                candidate.pop();
                return;
            }
            Self::combination(k, n, candidate, cur_sum + num, num + 1, ret);
            candidate.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::combination_sum3(3, 7));
    }
}
