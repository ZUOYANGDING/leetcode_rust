use std::cmp::min;

struct Solution;
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        if s.len() < 2 {
            return s;
        }
        let mut ret: Vec<char> = s.chars().collect();
        let k = k as usize;
        let mut idx = 0;
        if s.len() <= 2 * k {
            let right_bound = std::cmp::min(k - 1, s.len() - 1);
            Self::reverse_help(&mut ret, 0, right_bound);
            return ret.iter().collect::<String>();
        }
        while idx < s.len() {
            if idx + k < s.len() {
                Self::reverse_help(&mut ret, idx, idx + k - 1);
            } else {
                Self::reverse_help(&mut ret, idx, s.len() - 1);
            }
            idx += k * 2;
        }
        return ret.iter().collect::<String>();
    }

    pub fn reverse_help(s: &mut Vec<char>, mut left_ptr: usize, mut right_ptr: usize) {
        if s.len() < 2 {
            return;
        }
        while left_ptr < right_ptr {
            let tmp = s[left_ptr];
            s[left_ptr] = s[right_ptr];
            s[right_ptr] = tmp;
            left_ptr += 1;
            right_ptr -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("abcdefg");
        println!("{:?}", Solution::reverse_str(s, 4));
    }
}
