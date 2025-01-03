use std::alloc::handle_alloc_error;

struct Solution;

impl Solution {
    pub fn reverse_words_naive(s: String) -> String {
        let words: Vec<String> = s
            .split_whitespace()
            .into_iter()
            .map(|word| word.to_string())
            .collect();
        let mut ret = String::new();
        let mut idx = words.len() - 1;
        while idx > 0 {
            ret.push_str(&words[idx]);
            ret.push_str(" ");
            idx -= 1;
        }
        ret.push_str(&words[idx]);
        ret
    }

    pub fn reverse_words(s: String) -> String {
        let mut s_vec = s.as_bytes().to_vec();
        Self::trim_extra_space(&mut s_vec);
        Self::reverse_string(&mut s_vec);
        Self::reverse_each_word(&mut s_vec);
        String::from_utf8(s_vec).unwrap()
    }

    pub fn trim_extra_space(s: &mut Vec<u8>) {
        let head_idx = 0;
        while s[head_idx] == ' ' as u8 {
            s.remove(head_idx);
        }
        let mut tail_idx = s.len() - 1;
        while s[tail_idx] == ' ' as u8 {
            s.remove(tail_idx);
            tail_idx = s.len() - 1;
        }
        if s.len() < 2 {
            return;
        }
        let mut fast_ptr = 1;
        let mut slow_ptr = 0;
        while fast_ptr < s.len() {
            if s[slow_ptr] == ' ' as u8 && s[fast_ptr] == ' ' as u8 {
                while fast_ptr < s.len() && s[fast_ptr] == ' ' as u8 {
                    s.remove(fast_ptr);
                }
                slow_ptr = fast_ptr;
                fast_ptr += 1;
            } else {
                slow_ptr += 1;
                fast_ptr += 1;
            }
        }
    }

    pub fn reverse_string(s: &mut Vec<u8>) {
        if s.len() < 2 {
            return;
        }
        let mut head_ptr = 0;
        let mut tail_ptr = s.len() - 1;
        while head_ptr < tail_ptr {
            let temp = s[head_ptr];
            s[head_ptr] = s[tail_ptr];
            s[tail_ptr] = temp;
            head_ptr += 1;
            tail_ptr -= 1;
        }
    }

    pub fn reverse_each_word(s: &mut Vec<u8>) {
        if s.len() < 2 {
            return;
        }
        let mut slow_ptr = 0;
        let mut fast_ptr = 1;
        while fast_ptr < s.len() {
            while fast_ptr < s.len() && s[fast_ptr] != ' ' as u8 {
                fast_ptr += 1;
            }
            let mut head_ptr = slow_ptr;
            let mut tail_ptr = fast_ptr - 1;
            while head_ptr < tail_ptr {
                let temp = s[head_ptr];
                s[head_ptr] = s[tail_ptr];
                s[tail_ptr] = temp;
                head_ptr += 1;
                tail_ptr -= 1;
            }
            slow_ptr = fast_ptr + 1;
            fast_ptr = slow_ptr + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = " he  llo    w orld ".to_string();
        let ret = Solution::reverse_words(s);
        println!("{:?}", ret);
    }
}
