use std::cmp;

struct Solution;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut v_1 = vec![0; 26];
        let word = words[0].to_owned();
        for w in word.as_bytes() {
            v_1[(w - 97) as usize] += 1;
        }
        for idx in 1..words.len() {
            let word = words[idx].to_owned();
            let mut v_other = vec![0; 26];
            for w in word.as_bytes() {
                v_other[(w - 97) as usize] += 1;
            }
            for pos in 0..26 {
                v_1[pos] = cmp::min(v_1[pos], v_other[pos])
            }
        }
        let mut ret = Vec::new();
        for idx in 0..26 {
            let s = ((idx as u8 + 97) as char).to_string();
            for _ in 0..v_1[idx] {
                ret.push(s.to_owned());
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = vec![
            String::from("cool"),
            String::from("lock"),
            String::from("cook"),
        ];
        let result = Solution::common_chars(words.to_owned());
        println!("{:?}", result);
    }
}
