/**
 * Leecode 1005
 */
struct Solution;
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_by(|a, b| b.abs().cmp(&a.abs()));
        for val in nums.iter_mut() {
            if *val < 0 && k > 0 {
                *val *= -1;
                k -= 1;
            }
        }
        if k % 2 == 1 {
            *nums.last_mut().unwrap() *= -1;
        }
        nums.iter().sum()
    }
}
fn main() {
    assert_eq!(
        Solution::largest_sum_after_k_negations(vec![5, 6, 9, -3, 3], 2),
        20
    );
}
