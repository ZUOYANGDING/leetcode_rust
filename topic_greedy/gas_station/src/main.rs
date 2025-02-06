/**
 * Leetcode 134
 * Idea:
 *  Moving from gas station i to next gas station i+1, the res of gas in tank will be gas[i]-cost[i]
 *  The problem can be seperate as 2 parts:
 *  1. Is any possible to have a round trip. The solution is:
 *      If sum of all res from station 0 to last station is negative, means the gas provided by gas station
 *      does not support trip
 *  2. Where is the start point. The solution is:
 *      Assumpltion: Consider at station i, the sum of all res is negative, the start point must be at least i+1
 *      Reason: If the sum of res at station i is negative, and choose a start point between [0, i] like j, the gas
 *              cannot support the car arrive station i. Even though there might be a station k>i provides large amount
 *              of gas suppot the car to do the round trip, there is no enough gas support the car arrive the station k,
 *              since it cannot arrive station i and i<k.
 */
struct Solution;
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut cur_res = 0;
        let mut total_res = 0;
        let mut start = 0;
        for idx in 0..gas.len() {
            cur_res += gas[idx] - cost[idx];
            total_res += gas[idx] - cost[idx];
            if cur_res < 0 {
                cur_res = 0;
                start = idx + 1;
            }
        }
        if total_res < 0 {
            return -1;
        } else {
            return start as i32;
        }
    }
}
fn main() {
    assert_eq!(
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
        -1
    )
}
