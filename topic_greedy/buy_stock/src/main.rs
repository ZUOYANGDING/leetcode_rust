struct Solution;
impl Solution {
    // Leetcode 122
    // The max share can be hold is 1, which means if buy at new price, need to sell at the same day
    // Therefore, any profit from day i to j can be seperated as
    //  price[j]-price[i] =
    //      (price[j] - price[j-1]) +
    //      (price[j-1]- price[j-2]) +
    //              ...              +
    //      (price[j-i+1] - price[i])
    // Therefore, just need to calculate the price diff each day, and collect the positive ones
    pub fn max_profit_II(prices: Vec<i32>) -> i32 {
        let mut profit = vec![];
        for idx in 1..prices.len() {
            profit.push(prices[idx] - prices[idx - 1]);
        }
        let mut ret = 0;
        for idx in 0..profit.len() {
            if profit[idx] > 0 {
                ret += profit[idx];
            }
        }
        ret
    }

    // Leetcode 121
    // Dynamically record the lowest price until the day i
    // then decide if need to buy at the lowest price day and sell at day i
    // Important: the day with lowest price is not the global lowest, it is a lowest until day i
    pub fn max_profit_I(prices: Vec<i32>) -> i32 {
        let mut lowest = prices[0];
        let mut max_profit = 0;

        for idx in 1..prices.len() {
            if prices[idx] < lowest {
                lowest = prices[idx];
            } else if max_profit < prices[idx] - lowest {
                max_profit = prices[idx] - lowest;
            }
        }
        max_profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(7, Solution::max_profit_II(prices));
}
