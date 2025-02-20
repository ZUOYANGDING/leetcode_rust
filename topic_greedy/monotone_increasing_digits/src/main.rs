/**
 * Leetcode 738
 * Idea: If a number is not monotone increasing at index i,
 *       which means digits[i]>digits[i+1].
 *       To change it to the largest monotone increasing number smaller than it,
 *          digits[i] = digits[i]-1;
 *          digits[i+1] = 9
 *       However, if go throght from highest position (index=0), it might cause a problem "a change in digits[i] might trigger the change in digits[i-1]"
 *       Therefore, check the number from lowest position is a good choice
 */
struct Solution;
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        if n < 10 {
            return n;
        }
        let mut digits = n.to_string().into_bytes();
        let mut idx = digits.len() - 1;
        // record the position that all digits after it should set as '9'
        let mut nine_start = digits.len();
        while idx > 0 {
            if digits[idx] < digits[idx - 1] {
                nine_start = idx;
                digits[idx - 1] -= 1;
            }
            idx -= 1;
        }
        for digit in digits.iter_mut().skip(nine_start) {
            *digit = 57;
        }
        println! {"{:?}", digits};
        digits
            .into_iter()
            .fold(0, |acc, digit| acc * 10 + ((digit - 48) as i32))
    }
}
fn main() {
    assert_eq!(99, Solution::monotone_increasing_digits(100));
    assert_eq!(1234, Solution::monotone_increasing_digits(1234));
    assert_eq!(299, Solution::monotone_increasing_digits(332));
}
