/**
 * Leetcode 860
 */
struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five_count = 0;
        let mut ten_count = 0;
        for idx in 0..bills.len() {
            if bills[idx] == 5 {
                five_count += 1;
            } else if bills[idx] == 10 {
                five_count -= 1;
                ten_count += 1;
            } else {
                if ten_count > 0 {
                    ten_count -= 1;
                    five_count -= 1;
                } else {
                    five_count -= 3;
                }
            }
            if five_count < 0 || ten_count < 0 {
                return false;
            }
        }
        true
    }
}
fn main() {
    assert_eq!(
        Solution::lemonade_change(vec![5, 5, 5, 5, 20, 20, 5, 5, 20, 5]),
        false
    )
}
