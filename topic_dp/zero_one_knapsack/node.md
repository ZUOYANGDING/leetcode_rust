### Knapsack Problem

#### Basic Question
- 01 knapsack: There are m **items (each item just have one)** and a knapsack which can contains weight as n. Each item weight as weight[i] and have value value[i]. Get the max value of the knapsack can contains.
- full knapsack: There are m **different kind of items (each kind of items is infinity number)** and a knapsack which can contains weight as n. Each **KIND of** item weight as weight[i] and have value value[i]. Get the max value of the knapsack can contains. 

#### Basic analysis
- For 01 knapsack: For each item it can be choose or not. Therefore, the bruteforce can be applied as `trace_back`, and the complexity is O(2^n). So, try to find a dp way to decrease the complexity. To decide if choose the item i, two info need to be known: 
  - the max value of the knapsack before/do not choose the item i.
  - Suppose the weight after choose item i is j, if choose the item i (the weight does not exceed the weight j), the max value of of knapsack with weight j-weight[i].
- For full knapsack: For each item kind, we can pick as many as we can. Comparing to 01 knapsack above, the condition become:
  - the max value of the knapsack do not choose the kind i item at all (only pick among 0..i-1)
  - Suppose the weight after picking kind i item to make the knapsack weight is j, the max value of knapsack with weight j-weight[i]

#### Idea of 01 knapsack
- Define the DP array: dp[i][j], which means the max value of the knapsack with weight j and choose item from 0 to i. Here knapsack with weight j does not means the sum of items' weight is j, it means the knapsack can contain weight j.
- To decide the dp[i][j], based on basic analysis, it should be 2 condition, choose item i or not:
  - do not choose item i: dp[i-1][j]
  - choose item i: dp[i-1][j-weight[i]] + value[i]
- Get the dp formula: dp[i][j] = max(dp[i-1][j], dp[i-1][j-weight[i]] + value[i])
- Consider the init:
  - for i == 0, j in 0..n, dp[0][j] = value[0]. Which means choose item 0, value of knapsack always be value[0].
  - for j == 0, i in 0..m, dp[i][0] = 0. Which means the knapsack with weight 0, value of knapsack always be 0.
- Make it simple (compress from 2D dp array to 1D dp array):
  - Our target is to record the max value of knapsack with each weight 0..j..n. Therefore, it does not need to record to choose item 0 to item i for each weight j (just record the max of them).
  - compare the 2D array to 1D array as dp[j] which record the max value of knapsack with weight j
  - To decide dp[j]:
    - do not choose a new item i: dp[j]
    - choose a new item i: dp[j-weight[i]] + value[i]
  - init dp: when j==0, means the weight of the knapsack is 0, so the value of it is 0;
  - Still use 2 iteration:
    - iteration order : cannot be iter 1..m as outer and then iter 0..n as inner:
      - Example:
        -  3 items with weight [1, 3, 4] and value [15, 20, 30]; knapsack weight is 4
        - for dp[2] = max(dp[2-weight[0]] + value[0], dp[2]), in this case the item 0 count twice

#### Idea of full knapsack
- Define the DP array: dp[i][j], which means the max value of knapsack with weight j and pick item from item kind 0..i
- The formula is:
  - dp[i][j] = dp[i-1][j], when do not pick kind i item at all
  - dp[i][j] = dp[i][j-weight[i]] + value[i], when pick kind i item (**Not i-1, because there is infinity number of item i dp[i][j-weight[i]] means the max value when do not pick the current object of kind i item**)
- Init:
  - i==0: dp[0][j] = (num of item 0 which sum less than j) * value of 0
  - j==0: dp[i][0] = 0, which means the knapsack with weight 0, value of knapsack always be 0. 
- Compress dp[i][j] into dp[j]