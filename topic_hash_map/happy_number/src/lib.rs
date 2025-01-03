use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        let mut result_set = HashSet::new();
        let mut new_number = n;
        loop {
            let mut sum_up = 0;
            while new_number / 10 > 0 {
                sum_up += (new_number % 10) * (new_number % 10);
                new_number = new_number / 10;
            }
            sum_up += new_number * new_number;
            if sum_up == 1 {
                return true;
            }
            if result_set.contains(&sum_up) {
                return false;
            } else {
                result_set.insert(sum_up);
                new_number = sum_up;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_happy(19), true);
    }
}
