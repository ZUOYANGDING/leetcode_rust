/**
 * Leetcode 509
 */
struct Solution;
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut n_1 = 1;
        let mut n_2 = 0;
        for _ in 2..=n {
            let tmp = n_1;
            n_1 = n_1 + n_2;
            n_2 = tmp;
        }
        n_1
    }
}
fn main() {
    assert_eq!(3, Solution::fib(4));
}
