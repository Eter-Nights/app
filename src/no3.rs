struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = Vec::from(s);
        let mut window = [false; 128];
        let mut left = 0;
        let mut result = 0;

        for (index, &c) in s.iter().enumerate() {
            let c = c as usize;

            while window[c] {
                window[s[left] as usize] = false;
                left += 1;
            }

            window[c] = true;
            result = result.max(index - left + 1);
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }

    #[test]
    fn it_works2() {
        let s = "bbbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
    }

    #[test]
    fn it_works3() {
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
