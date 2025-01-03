struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let hey_vec = haystack.as_bytes().iter().map(|c| *c).collect::<Vec<u8>>();
        let needle_vec = needle.as_bytes().iter().map(|c| *c).collect::<Vec<u8>>();
        let mut next = vec![0; needle_vec.len()];
        Self::create_next(&needle_vec, &mut next);
        let mut j: i32 = 0;
        for idx in 0..hey_vec.len() {
            while j > 0 && hey_vec[idx] != needle_vec[j as usize] {
                j = next[(j - 1) as usize];
            }
            if hey_vec[idx] == needle_vec[j as usize] {
                j += 1;
            }
            if j == needle_vec.len() as i32 {
                return idx as i32 - j + 1;
            }
        }
        -1
    }

    fn create_next(needle: &Vec<u8>, next: &mut [i32]) {
        let mut j: i32 = 0;
        next[j as usize] = 0;
        for idx in 1..needle.len() {
            while j > 0 && needle[j as usize] != needle[idx] {
                j = next[(j - 1) as usize];
            }
            if needle[idx] == needle[j as usize] {
                j += 1;
            }
            next[idx] = j;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let hey = String::from("aabaabaafa");
        let needle = String::from("aabaaf");
        println!("{:?}", Solution::str_str(hey, needle));
    }
}
