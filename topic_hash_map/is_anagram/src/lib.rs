use std::collections::{hash_map::Entry, HashMap};
struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut v = vec![0; 26];
        for c in s.as_bytes().into_iter() {
            v[(c - 97) as usize] += 1;
        }
        for c in t.as_bytes().into_iter() {
            v[(c - 97) as usize] -= 1;
        }
        v.iter().all(|n| *n == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert_eq!(Solution::is_anagram(s.to_owned(), t.to_owned()), true);
    }
}
