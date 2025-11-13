use super::definition::ListNode;
use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vals = VecDeque::new();
        let mut cur = head;
        while let Some(node) = cur {
            vals.push_back(node.val);
            cur = node.next;
        }
        while !vals.is_empty() {
            if vals.front() == vals.back() {
                vals.pop_front();
                vals.pop_back();
                continue;
            } else {
                break;
            }
        }
        vals.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let root = ListNode::from_vec(vec![1, 2, 2, 1]);
        assert_eq!(Solution::is_palindrome(root), true);
    }
}
