/**
 * Leetcode 131
 */
struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut candidate = Vec::new();
        let mut ret = Vec::new();
        Self::trace_back(&s, 1, &mut candidate, &mut ret);
        ret
    }

    fn trace_back(
        s: &String,
        cur_idx: usize,
        candidate: &mut Vec<String>,
        ret: &mut Vec<Vec<String>>,
    ) {
        if cur_idx > s.len() {
            ret.push(candidate.clone());
            return;
        }

        for idx in cur_idx..=s.len() {
            let slice = &s[cur_idx - 1..idx];
            // No need to check the palindrome until add all sliced substring
            // if the current sliced one is not a palindrome, just skip the whole sub-tree
            if !Self::check_palindrome(slice.as_bytes()) {
                continue;
            }
            candidate.push(slice.to_string());
            Self::trace_back(s, idx + 1, candidate, ret);
            candidate.pop();
        }
    }

    fn check_palindrome(s: &[u8]) -> bool {
        let mut head_ptr = 0;
        let mut tail_ptr = s.len() - 1;
        while head_ptr < tail_ptr {
            if s[head_ptr] != s[tail_ptr] {
                return false;
            }
            head_ptr += 1;
            tail_ptr -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aab";
        println!("{:?}", Solution::partition(s.to_string()));
    }
}
