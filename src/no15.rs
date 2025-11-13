struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum > 0 {
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    while right > left && nums[left] == nums[left + 1] {
                        left += 1
                    }
                    while right > left && nums[right] == nums[right - 1] {
                        right -= 1
                    }

                    left += 1;
                    right -= 1;
                }
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            Solution::three_sum(nums),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn it_works2() {
        let nums = vec![0, 1, 1];
        let answer: Vec<Vec<_>> = vec![];
        assert_eq!(Solution::three_sum(nums), answer);
    }

    #[test]
    fn it_works3() {
        let nums = vec![0, 0, 0];
        assert_eq!(Solution::three_sum(nums), vec![vec![0, 0, 0]]);
    }
}
