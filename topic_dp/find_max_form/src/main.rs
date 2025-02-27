/**
 * Leetcode 474
 *
 * Idea:
 *       This question can be transformed to a knapsack problem with 2D weight (m, n). And each value of the item is 1.
 *       Therefore, the dp[i][j][k] can be define as recording the largest value of the knapsack when select number among 0..i and knapsack weight is (j, k)
 *       Then the formula is:
 *              dp[i][j][k] = dp[i-1][j][k], where j<weight[i].0 or k<weight[i].1;
 *              dp[i][j][k] = max(dp[i-1][j][k], dp[i-1][j-weight[i].0][k-weight[i].1), where j>=weight[i].0 and k>=weight[i].1
 *       Init:
 *              i==0: use item 0 to fill the knapsack, so set all dp[0][j][k]=1 when j>=weight[0].0 and k>=weight[0].1
 *              j==0 and k==0: use item 0..i to fill a knapsack with weight(0,0), so dp[i][0][0] = 0
 *       
 *       Improve: compress the 3D dp array into 2D dp array like 01 knapsack compress the 2D dp array into 1D dp array
 *       Therefore, define dp[j][k] to record the largest value of the knapsack with weight (j, k)
 *       Then the formula is:
 *              dp[j][k] = max(dp[j][k], dp[j-weight[i].0][k-weight[i].1])
 *
 *       Init: j==0 and k==0: use item 0..i to fill a knapsack weight(0,0). so dp[0][0] = 1
 */
struct Solution;
impl Solution {
    pub fn find_max_form_naive(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut count = vec![];
        for s in strs {
            let mut count_0 = 0;
            let mut count_1 = 0;
            for c in s.as_bytes() {
                if *c == b'1' {
                    count_1 += 1;
                } else {
                    count_0 += 1;
                }
            }
            count.push((count_0, count_1));
        }

        let mut dp = vec![vec![vec![0; (n + 1) as usize]; (m + 1) as usize]; count.len()];

        for i in 0..=m as usize {
            for j in 0..=n as usize {
                if count[0].0 <= i && count[0].1 <= j {
                    dp[0][i][j] = 1;
                }
            }
        }

        for i in 1..count.len() {
            for j in 0..=m as usize {
                for k in 0..=n as usize {
                    if k < count[i].1 as usize || j < count[i].0 as usize {
                        dp[i][j][k] = dp[i - 1][j][k]
                    } else {
                        let idx_0 = j - count[i].0 as usize;
                        let idx_1 = k - count[i].1 as usize;
                        dp[i][j][k] = std::cmp::max(dp[i - 1][j][k], dp[i - 1][idx_0][idx_1] + 1);
                    }
                }
            }
        }
        dp[count.len() - 1][m as usize][n as usize]
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut count = vec![];
        for s in strs {
            let mut count_0 = 0;
            let mut count_1 = 0;
            for c in s.as_bytes() {
                if *c == b'1' {
                    count_1 += 1;
                } else {
                    count_0 += 1;
                }
            }
            count.push((count_0, count_1));
        }

        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];

        for (zeros, ones) in count {
            for i in (zeros..=m as usize).rev() {
                for j in (ones..=n as usize).rev() {
                    dp[i][j] = std::cmp::max(dp[i][j], dp[i - zeros][j - ones] + 1);
                }
            }
        }
        dp[m as usize][n as usize]
    }
}
fn main() {
    assert_eq!(
        Solution::find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string()
            ],
            5,
            3
        ),
        4
    );

    assert_eq!(
        Solution::find_max_form_naive(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string()
            ],
            5,
            3
        ),
        4
    );
}
