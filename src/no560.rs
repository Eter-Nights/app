use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut pre_map = HashMap::new();
        pre_map.insert(0, 1);

        let mut pre = 0;

        for i in nums {
            pre += i;
            if pre_map.contains_key(&(pre - k)) {
                result += pre_map.get(&(pre - k)).unwrap();
            }
            pre_map
                .entry(pre)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::subarray_sum(nums, 3), 2);
    }
}
