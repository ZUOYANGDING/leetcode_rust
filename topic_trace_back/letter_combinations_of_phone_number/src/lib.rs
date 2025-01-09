/**
 * Leetcode 17
 */

/// Iteration of all digits
/// Recursion of each letter of the digit
struct Solution1;
impl Solution1 {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() < 1 {
            return vec![];
        }
        let mapping_arr: [Vec<String>; 8] = [
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            vec!["d".to_string(), "e".to_string(), "f".to_string()],
            vec!["g".to_string(), "h".to_string(), "i".to_string()],
            vec!["j".to_string(), "k".to_string(), "l".to_string()],
            vec!["m".to_string(), "n".to_string(), "o".to_string()],
            vec![
                "p".to_string(),
                "q".to_string(),
                "r".to_string(),
                "s".to_string(),
            ],
            vec!["t".to_string(), "u".to_string(), "v".to_string()],
            vec![
                "w".to_string(),
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
            ],
        ];

        let mut pool = Vec::new();
        for byte in digits.as_bytes() {
            let idx = byte - b'0' - 2;
            pool.push(mapping_arr[idx as usize].clone());
        }

        let mut candidate = Vec::new();
        let mut ret = Vec::new();
        Self::combination(&pool, pool.len() as i32, 0, &mut candidate, &mut ret);
        ret
    }

    fn combination(
        pool: &Vec<Vec<String>>,
        length: i32,
        cur_digit: i32,
        candidate: &mut Vec<String>,
        ret: &mut Vec<String>,
    ) {
        if candidate.len() == (length as usize) {
            let s: String = candidate.iter().map(|s| s.to_owned()).collect();
            ret.push(s.clone());
            return;
        }

        for digit in cur_digit..(pool.len() as i32) {
            for idx in 0..pool[digit as usize].len() {
                candidate.push(pool[digit as usize][idx].clone());
                Self::combination(pool, length, digit + 1, candidate, ret);
                candidate.pop();
            }
        }
    }
}

/// Iteration letters in each digit
/// Recursion the digit
struct Solution2;
impl Solution2 {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() < 1 {
            return vec![];
        }
        let map = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut candidate = String::new();
        let mut ret = vec![];
        Self::trace_back(&map, &digits, 0, &mut candidate, &mut ret);
        ret
    }

    fn trace_back(
        map: &[&str; 10],
        digits: &String,
        cur_idx: usize,
        candidate: &mut String,
        ret: &mut Vec<String>,
    ) {
        if candidate.len() == digits.len() {
            ret.push(candidate.clone());
            return;
        }
        let cur_map_idx = (digits.as_bytes()[cur_idx] - b'0') as usize;
        let cur_set = map[cur_map_idx];
        for c in cur_set.chars() {
            candidate.push(c);
            Self::trace_back(map, digits, cur_idx + 1, candidate, ret);
            candidate.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let digits = "23".to_string();
        let ret = Solution1::letter_combinations(digits.clone());
        println!("{:?}", ret);
        let ret_2 = Solution2::letter_combinations(digits);
        assert_eq!(ret, ret_2);
    }
}
