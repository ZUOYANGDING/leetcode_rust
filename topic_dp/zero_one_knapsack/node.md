### Knapsack Problem

#### Basic Question
- 01 knapsack: There are m **items (each item just have one)** and a knapsack which can contains weight as n. Each item weight as weight[i] and have value value[i]. Get the max value of the knapsack can contains.
- full knapsack: There are m **different kind of items (each kind of items is infinity number)** and a knapsack which can contains weight as n. Each **KIND of** item weight as weight[i] and have value value[i]. Get the max value of the knapsack can contains. 

#### Basic analysis
- For 01 knapsack: For each item it can be choose or not. Therefore, the bruteforce can be applied as `trace_back`, and the complexity is O(2^n). So, try to find a dp way to decrease the complexity. To decide if choose the item i, two info need to be known: 
  - the max value of the knapsack before/do not choose the item i.
  - Suppose the weight after choose item i is j, if choose the item i (the weight does not exceed the weight j), the max value of of knapsack with weight j-weight[i].
- 

#### Idea of 01 knapsack
- Define the DP array: dp[i][j], which means the max value of the knapsack with weight j and choose item from 0 to i. Here knapsack with weight j does not means the sum of items' weight is j, it means the knapsack can contain weight j.
- To decide the dp[i][j], based on basic analysis, it should be 2 condition, choose item i or not:
  - do not choose item i: dp[i-1][j]
  - choose item i: dp[i-1][j-weight[i]] + value[i]
- Get the dp formula: dp[i][j] = max(dp[i-1][j], dp[i-1][j-weight[i]] + value[i])
- Consider the init:
  - for i == 0, j in 0..n, dp[0][j] = value[0]. Which means choose item 0, value of knapsack always be value[0].
  - for j == 0, i in 0..m, dp[i][0] = 0. Which means the knapsack with weight 0, value of knapsack always be 0.
