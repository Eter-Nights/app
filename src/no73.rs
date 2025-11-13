use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = HashSet::new();
        let mut cols = HashSet::new();

        for (i, row) in matrix.iter().enumerate() {
            for (j, x) in row.iter().enumerate() {
                if *x == 0 {
                    rows.insert(i);
                    cols.insert(j);
                }
            }
        }

        for (i, row) in matrix.iter_mut().enumerate() {
            for (j, x) in row.iter_mut().enumerate() {
                if rows.contains(&i) || cols.contains(&j) {
                    *x = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]])
    }
}
