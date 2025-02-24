struct Solution;

impl Solution {
    fn zero_one_knapsack(
        item_array: &Vec<i32>,
        value_array: &Vec<i32>,
        knapsack_weight: usize,
    ) -> i32 {
        assert!(item_array.len() == value_array.len());
        let num_of_item = item_array.len();

        // init dp
        let mut dp = vec![vec![0; knapsack_weight + 1]; num_of_item];
        for idx in 0..=knapsack_weight {
            dp[0][idx] = value_array[0];
        }

        for i in 1..num_of_item {
            for j in 1..=knapsack_weight {
                if j < item_array[i] as usize {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = std::cmp::max(
                        dp[i - 1][j],
                        dp[i - 1][j - item_array[i] as usize] + value_array[i],
                    );
                }
            }
        }
        dp[num_of_item - 1][knapsack_weight]
    }

    fn zero_one_knapsack_one_dim_dp_array(
        item_array: &Vec<i32>,
        value_array: &Vec<i32>,
        knapsack_weight: usize,
    ) -> i32 {
        assert!(item_array.len() == value_array.len());
        let num_of_item = item_array.len();

        // init dp
        let mut dp = vec![0; knapsack_weight + 1];
        for i in 0..num_of_item {
            for j in (item_array[i] as usize..=knapsack_weight).rev() {
                // j < weight[i] do not need to be count
                dp[j] = std::cmp::max(dp[j], dp[j - item_array[i] as usize] + value_array[i]);
            }
        }

        dp[knapsack_weight]
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    input.clear();
    let _ = std::io::stdin().read_line(&mut input);
    let item_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(item_array.len() == numbers[0]);

    input.clear();
    let _ = std::io::stdin().read_line(&mut input);
    let value_array: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(value_array.len() == numbers[0]);

    println!(
        "{}",
        Solution::zero_one_knapsack_one_dim_dp_array(&item_array, &value_array, numbers[1])
    )
}
