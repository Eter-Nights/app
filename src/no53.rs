struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut min_pre = 0;
        let mut sum = 0;
        for i in nums {
            min_pre = min_pre.min(sum);
            sum += i;
            result = result.max(sum - min_pre);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
        assert_eq!(Solution::max_sub_array(nums), 6);
    }
}