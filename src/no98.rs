use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        let mut inorder = i64::MIN;

        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                if node.borrow().val as i64 <= inorder {
                    return false;
                }
                inorder = node.borrow().val as i64;
                root = node.borrow().right.clone();
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[2,1,3]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::is_valid_bst(root);
        assert_eq!(result, true);
    }
}
