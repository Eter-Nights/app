use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack = Vec::new();
        stack.push(root.clone());

        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();

                if left.is_some() {
                    stack.push(left.clone())
                }
                if right.is_some() {
                    stack.push(right.clone())
                }

                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
            }
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[4,2,7,1,3,6,9]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::invert_tree(root);
        let result = codec.serialize(result);
        assert_eq!(
            result,
            "[4,7,2,9,6,3,1]".to_string()
        );
    }
}
