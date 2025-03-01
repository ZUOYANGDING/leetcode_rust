struct Solution;

impl Solution {
    fn full_knapsack(items: Vec<i32>, values: Vec<i32>, knapsack: i32) -> i32 {
        assert!(items.len() == values.len());

        let mut dp = vec![vec![0; (knapsack + 1) as usize]; items.len()];

        // init
        for idx in items[0]..=knapsack {
            dp[0][idx as usize] = dp[0][(idx - items[0]) as usize] + values[0];
        }

        for i in 1..items.len() {
            for j in 1..=knapsack as usize {
                if j < items[i] as usize {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] =
                        std::cmp::max(dp[i - 1][j], dp[i][j - items[i] as usize] + values[i]);
                }
            }
            println!("{:?}", dp);
        }
        dp[items.len() - 1][knapsack as usize]
    }

    fn full_knapsack_1d_dp_array(items: Vec<i32>, values: Vec<i32>, knapsack: i32) -> i32 {
        assert!(items.len() == values.len());

        let mut dp = vec![0; (knapsack + 1) as usize];

        // init
        for idx in items[0]..=knapsack {
            dp[idx as usize] = dp[(idx - items[0]) as usize] + values[0];
        }

        for i in 1..items.len() {
            for j in items[i] as usize..=knapsack as usize {
                dp[j] = std::cmp::max(dp[j], dp[j - items[i] as usize] + values[i]);
            }
            println!("{:?}", dp);
        }
        dp[knapsack as usize]
    }
}
fn main() {
    let mut buf = String::new();

    std::io::stdin().read_line(&mut buf).unwrap();
    let nums: Vec<i32> = buf
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(nums.len() == 2);
    println!("Input the items weight...");
    buf.clear();

    std::io::stdin().read_line(&mut buf).unwrap();
    let items: Vec<i32> = buf
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(items.len() == nums[0] as usize);
    buf.clear();

    println!("Input the items value...");
    std::io::stdin().read_line(&mut buf).unwrap();
    let values: Vec<i32> = buf
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(values.len() == nums[0] as usize);
    buf.clear();

    println!(
        "{:?}",
        Solution::full_knapsack(items.clone(), values.clone(), nums[1])
    );
    println!(
        "{:?}",
        Solution::full_knapsack_1d_dp_array(items, values, nums[1])
    )
}
