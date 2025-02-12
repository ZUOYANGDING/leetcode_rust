/**
 * Leetcode 763
 *
 * Idea: Include as much different letters as we can inside each segment, but include for each segement, we need to contains as less letters as we can
 * Therefore, we can record the start and end index of each letter showing up in the string
 * Then the problem tranforms to a problem: How many non-overlaping segments exists inside the the String, which is similar to Leetcode 435
 * But the different is we need to update both and right bound of the segments, since we need to use them to calculate the length of each segment.
 */
struct Solution;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        if s.len() == 1 {
            return vec![1];
        }
        // Array to act as hashmap to store the start and end index
        let mut index_array = [(-1, -1); 26];
        for (idx, c) in s.as_bytes().iter().enumerate() {
            if index_array[(c - b'a') as usize].0 == -1 && index_array[(c - b'a') as usize].1 == -1
            {
                index_array[(c - b'a') as usize].0 = idx as i32;
            } else if index_array[(c - b'a') as usize].0 != -1
                && index_array[(c - b'a') as usize].1 == -1
            {
                index_array[(c - b'a') as usize].1 = idx as i32;
            } else {
                let target_idx = std::cmp::max(idx as i32, index_array[(c - b'a') as usize].1);
                index_array[(c - b'a') as usize].1 = target_idx;
            }
        }
        // sort by the left bound
        // since the origin index_array is alphabetical ordered from 'a' to 'z', but the letter in string is not.
        index_array.sort_by(|a, b| (a.0).cmp(&b.0));
        // filter the index_array to drop the letter does not show in letter, and set the right bound as left bound if the letter only show one time
        let mut index_vec = vec![];
        for idx in 0..26 {
            if index_array[idx].0 != -1 {
                if index_array[idx].1 == -1 {
                    index_vec.push((index_array[idx].0, index_array[idx].0));
                } else {
                    index_vec.push((index_array[idx].0, index_array[idx].1));
                }
            }
        }

        let mut left_bound = index_vec[0].0;
        let mut right_bound = index_vec[0].1;
        let mut ret = vec![];
        for idx in 0..index_vec.len() {
            if index_vec[idx].0 > right_bound {
                ret.push(right_bound - left_bound + 1);
                left_bound = index_vec[idx].0;
                right_bound = index_vec[idx].1;
            } else if index_vec[idx].1 > right_bound {
                right_bound = index_vec[idx].1;
            }
        }
        ret.push(right_bound - left_bound + 1);
        ret
    }
}
fn main() {
    assert_eq!(
        Solution::partition_labels(String::from("ababcbacadefegdehijhklij")),
        vec![9, 7, 8]
    );
}
