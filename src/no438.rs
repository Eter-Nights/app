struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut window = [0; 26];
        let mut result = vec![];
        let mut left = 0;
        let s = s.as_bytes();

        for c in p.bytes() {
            window[(c - b'a') as usize] += 1;
        }

        for (right, c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            window[c] -= 1;

            while window[c] < 0 {
                window[(s[left] - b'a') as usize] += 1;
                left += 1;
            }

            if right + 1 - left == p.len() {
                result.push(left as i32);
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
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();

        assert_eq!(Solution::find_anagrams(s, p), vec![0, 6]);
    }
}
