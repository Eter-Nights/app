struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut left_product = 1;
        let mut right_product = 1;
        let mut left = 0;
        let mut right = nums.len();

        for _ in 0..nums.len() {
            right -= 1;

            result[left] *= left_product;
            result[right] *= right_product;

            left_product *= nums[left];
            right_product *= nums[right];

            left += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::product_except_self(nums), vec![24, 12, 8, 6]);
    }
}
