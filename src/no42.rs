struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_left = 0;
        let mut max_right = 0;

        while left < right {
            max_left = max_left.max(height[left]);
            max_right = max_right.max(height[right]);
            if height[left] < height[right] {
                result += max_left - height[left];
                left += 1;
            } else {
                result += max_right - height[right];
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
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn it_works2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }
}
