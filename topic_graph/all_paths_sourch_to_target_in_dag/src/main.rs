/**
 * Leetcode 797
 */
struct Solution;
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut cur_path = vec![];
        cur_path.push(0);
        Self::dfs(&graph, &mut ret, &mut cur_path, 0);
        ret
    }

    fn dfs(
        graph: &Vec<Vec<i32>>,
        ret: &mut Vec<Vec<i32>>,
        cur_path: &mut Vec<i32>,
        cur_node_idx: usize,
    ) {
        if cur_node_idx == graph.len() - 1 {
            ret.push(cur_path.clone());
            return;
        }

        for node_idx in graph[cur_node_idx].clone() {
            cur_path.push(node_idx);
            Self::dfs(graph, ret, cur_path, node_idx as usize);
            cur_path.pop();
        }
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let nums: Vec<usize> = buffer
        .split_whitespace()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(nums.len() == 2);
    let (num_of_nodes, num_of_edges) = (nums[0], nums[1]);
    let mut graph = vec![vec![]; num_of_nodes];
    buffer.clear();
    for _ in 0..num_of_edges {
        let _ = std::io::stdin().read_line(&mut buffer);
        let edge: Vec<i32> = buffer
            .split_whitespace()
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();
        graph[(edge[0] - 1) as usize].push(edge[1] - 1);
        buffer.clear();
    }
    println!("{:?}", graph);
    let ret = Solution::all_paths_source_target(graph);
    println!("{:?}", ret);
}
