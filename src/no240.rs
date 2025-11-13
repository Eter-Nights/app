struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut x = 0;
        let mut y = n;

        while x < m && y > 0 {
            if matrix[x][y - 1] == target {
                return true;
            } else if matrix[x][y - 1] < target {
                x += 1;
            } else {
                y -= 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
        ];
        assert_eq!(Solution::search_matrix(matrix, 9), true);
    }
}
