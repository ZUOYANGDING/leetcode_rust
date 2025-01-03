struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_dict = [0; 26];
        for c in magazine.as_bytes() {
            magazine_dict[(c - 97) as usize] += 1;
        }
        for c in ransom_note.as_bytes() {
            if magazine_dict[(c - 97) as usize] - 1 < 0 {
                return false;
            } else {
                magazine_dict[(c - 97) as usize] -= 1;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aab");
        println!("{:?}", Solution::can_construct(ransom_note, magazine))
    }
}
