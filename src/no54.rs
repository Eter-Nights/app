struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = matrix.len();
        let mut n = matrix[0].len();
        let size = m * n;
        let mut result = Vec::with_capacity(m * n);
        let mut i = 0;
        let mut j = -1;
        let mut di = 0;

        while result.len() < size {
            let (dx, dy) = Self::DIRS[di];
            for _ in 0..n {
                i += dx;
                j += dy;
                result.push(matrix[i as usize][j as usize]);
            }
            di = (di + 1) % 4;
            (n, m) = (m - 1, n);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }
}
