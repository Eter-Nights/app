use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.into_iter().enumerate() {
            if let Some(&i) = map.get(&(target - num)) {
                return vec![i as i32, index as i32];
            } else {
                map.insert(num, index);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}
