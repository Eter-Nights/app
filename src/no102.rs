use super::definition::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut deque = VecDeque::new();
        if let Some(node) = root {
            deque.push_back(node);
        }

        while !deque.is_empty() {
            let mut tmp = vec![];
            for _ in 0..deque.len() {
                if let Some(node) = deque.pop_front() {
                    tmp.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        deque.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        deque.push_back(right);
                    }
                }
            }
            result.push(tmp);
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
        let data = "[3,9,20,null,null,15,7]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::level_order(root);
        assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}
