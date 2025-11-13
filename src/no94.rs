use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut stack = vec![];

        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                root = node.borrow().right.clone();
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
        let data = "[1,null,2,3]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::inorder_traversal(root);
        assert_eq!(result, vec![1, 3, 2]);
    }
}
