use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push(node.clone());
        }
        let mut pre: Option<Rc<RefCell<TreeNode>>> = None;

        while let Some(node) = stack.pop() {
            if let Some(pre) = pre.clone() {
                pre.borrow_mut().right = Some(node.clone());
            }

            if let Some(right) = node.borrow_mut().right.take() {
                stack.push(right);
            }
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push(left);
            }

            pre = Some(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[1,2,5,3,4,null,6]".to_string();
        let mut root = codec.deserialize(data);
        Solution::flatten(&mut root);
        assert_eq!(
            codec.serialize(root),
            "[1,null,2,null,3,null,4,null,5,null,6]".to_string()
        );
    }
}
