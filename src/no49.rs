use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for str in strs {
            let mut arry = [0; 26];
            str.bytes().for_each(|b| {arry[(b - b'a') as usize] += 1});
            map.entry(arry).or_insert(vec![]).push(str);
        }

        map.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let strs = ["eat","tea","tan","ate","nat","bat"].iter().map(|s| s.to_string()).collect();
        let mut result = Solution::group_anagrams(strs);
        for i in &mut result {
            i.sort();
        }
        result.sort_by_key(|i|i.len());

        let answer = [vec!["bat"], vec!["nat","tan"],vec!["ate","eat","tea"]]
            .into_iter()
            .map(|s| s.into_iter().map(|s| s.to_string()).collect())
            .collect::<Vec<Vec<String>>>();

        assert_eq!(result, answer);
    }
}