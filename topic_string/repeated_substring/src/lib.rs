struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut end_ptr = 1;
        while end_ptr < s.len() {
            let slice = &s[0..end_ptr];
            let mut start_pos = 0;
            let mut ret = true;
            while start_pos + end_ptr <= s.len() {
                if &s[start_pos..start_pos + end_ptr] == slice {
                    start_pos += end_ptr;
                } else {
                    ret = false;
                    break;
                }
            }
            if ret && s.len() % end_ptr == 0 {
                return ret;
            } else {
                end_ptr += 1;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("abac");
        println!("{:?}", Solution::repeated_substring_pattern(s));
    }
}
