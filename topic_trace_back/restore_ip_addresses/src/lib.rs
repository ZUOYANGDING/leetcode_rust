/**
 * Leetcode 93
 */
struct Solution;
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut candidate = String::new();
        let mut ret = Vec::new();
        Self::trace_back(&s, 0, 1, &mut candidate, &mut ret);
        ret
    }

    fn trace_back(
        s: &String,
        cur_slice: usize,
        cur_idx: usize,
        candidate: &mut String,
        ret: &mut Vec<String>,
    ) {
        if cur_idx > s.len() {
            if cur_slice != 4 {
                return;
            }
            ret.push(candidate.clone());
            return;
        }

        for idx in cur_idx..=s.len() {
            if idx - (cur_idx - 1) >= 4 {
                return;
            }
            let slice = &s[cur_idx - 1..idx];
            if !Self::validation(slice) {
                continue;
            }
            let origin_candidate = candidate.clone();
            if idx == s.len() {
                candidate.push_str(slice);
            } else {
                candidate.push_str(slice);
                candidate.push_str(".");
            }
            Self::trace_back(s, cur_slice + 1, idx + 1, candidate, ret);
            *candidate = origin_candidate.clone();
        }
    }

    fn validation(s: &str) -> bool {
        let s_in_bytes = s.as_bytes();
        if s_in_bytes.len() > 1 && s_in_bytes[0] == b'0' {
            return false;
        }
        let s_in_number = s.parse::<usize>().unwrap();
        if s_in_number > 255 {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("25525511135");
        println!("{:?}", Solution::restore_ip_addresses(s));
    }
}
