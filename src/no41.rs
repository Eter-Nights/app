struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] < n as i32 && nums[nums[i] as usize - 1] != nums[i] {
                let tmp = nums[i] as usize - 1;
                nums.swap(i, tmp);
            }
        }

        for (i, x) in nums.iter().enumerate() {
            if *x != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        n as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 0];
        assert_eq!(Solution::first_missing_positive(nums), 3);
    }
}
