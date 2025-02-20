/**
 * Leetcode 70, similar to 509 (fib)
 */
struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let mut n_1 = 2;
        let mut n_2 = 1;

        for _ in 3..=n {
            let tmp = n_1;
            n_1 = n_1 + n_2;
            n_2 = tmp;
        }
        n_1
    }
}
fn main() {
    assert_eq!(3, Solution::climb_stairs(3));
}
