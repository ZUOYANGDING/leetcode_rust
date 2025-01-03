struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut start_row = 0;
        let mut start_col = 0;
        let mut ret = vec![vec![0; n as usize]; n as usize];
        let mut number = 1;
        let mut round = n / 2; // how many round will loop
        while round > 0 {
            let mut row = start_row;
            let mut col = start_col;
            while col < n - start_col - 1 {
                ret[row as usize][col as usize] = number;
                number += 1;
                col += 1;
            }
            while row < n - start_row - 1 {
                ret[row as usize][col as usize] = number;
                number += 1;
                row += 1;
            }
            while col > start_col {
                ret[row as usize][col as usize] = number;
                number += 1;
                col -= 1;
            }
            while row > start_row {
                ret[row as usize][col as usize] = number;
                number += 1;
                row -= 1;
            }
            start_row += 1;
            start_col += 1;
            round -= 1;
        }
        if n % 2 != 0 {
            let center = (n / 2) as usize;
            ret[center][center] = number;
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ans = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];
        let result = Solution::generate_matrix(5);
        for i in 1..5 {
            for j in 1..5 {
                assert_eq!(result[i as usize][j as usize], ans[i as usize][j as usize]);
            }
        }
    }
}
