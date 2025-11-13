struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by_key(|a| a[0]);
        let mut result: Vec<Vec<i32>> = Vec::new();

        for interval in intervals {
            if result.is_empty() || result.last().unwrap()[1] < interval[0] {
                result.push(interval);
            } else {
                let last_interval = result.last_mut().unwrap();
                last_interval[1] = last_interval[1].max(interval[1]);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
