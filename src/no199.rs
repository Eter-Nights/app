use super::definition::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut deque = VecDeque::new();
        if let Some(node) = root {
            deque.push_back(node);
        }

        while !deque.is_empty() {
            result.push(deque.back().unwrap().borrow().val);
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    if let Some(left) = node.borrow().left.clone() {
                        deque.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        deque.push_back(right);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[1,2,3,null,5,null,4]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::right_side_view(root);
        assert_eq!(result, vec![1, 3, 4]);
    }
}
