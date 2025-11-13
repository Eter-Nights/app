struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result = 0;

        while left < right {
            result = result.max((right - left) as i32 * height[right].min(height[left]));
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
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
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn it_works2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
