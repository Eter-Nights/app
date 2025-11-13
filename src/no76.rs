struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let mut cnt = [0; 128];
        let mut less = 0;

        let mut result_left = 0;
        let mut result_right = s_bytes.len();

        for i in t.bytes() {
            if cnt[i as usize] == 0 {
                less += 1;
            }
            cnt[i as usize] += 1;
        }

        let mut left = 0;
        for (right, &c) in s_bytes.iter().enumerate() {
            let c = c as usize;
            cnt[c] -= 1;
            if cnt[c] == 0 {
                less -= 1;
            }

            while less == 0 {
                if right - left < result_right - result_left {
                    (result_left, result_right) = (left, right);
                }

                let left_c = s_bytes[left] as usize;
                if cnt[left_c] == 0 {
                    less += 1;
                }
                cnt[left_c] += 1;
                left += 1;
            }
        }
        if result_right < s_bytes.len() {
            String::from(&s[result_left..=result_right])
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();

        assert_eq!(Solution::min_window(s, t), "BANC".to_string());
    }
}
