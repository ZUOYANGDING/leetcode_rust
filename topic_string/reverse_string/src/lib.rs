struct Solution;
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() < 2 {
            return;
        }
        let mut left_ptr = 0;
        let mut right_prt = s.len() - 1;
        while left_ptr < right_prt {
            let temp_char = s[left_ptr];
            s[left_ptr] = s[right_prt];
            s[right_prt] = temp_char;
            left_ptr += 1;
            right_prt -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut v);
        println!("{:?}", v);
    }
}
