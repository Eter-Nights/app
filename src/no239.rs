use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut q = VecDeque::new();
        let k = k as usize;

        for i in 0..nums.len() {
            while !q.is_empty() && nums[*q.back().unwrap()] <= nums[i] {
                q.pop_back();
            }
            q.push_back(i);
            if i - *q.front().unwrap() >= k {
                q.pop_front();
            }
            if i >= k - 1 {
                result.push(nums[*q.front().unwrap()]);
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
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(
            Solution::max_sliding_window(nums, 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
