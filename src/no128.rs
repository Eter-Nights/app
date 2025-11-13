use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut result = 0;

        for &x in &set {
            if !set.contains(&(x - 1)) {
                let mut cur_length = 1;
                let mut number = x;

                while set.contains(&(number + 1)) {
                    number += 1;
                    cur_length += 1;
                }

                result = result.max(cur_length);
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
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn it_works2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }
}
