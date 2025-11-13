use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        while preorder.is_empty() {
            return None;
        }
        let root = Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        let mut stack = vec![root.clone()];
        let mut inorder_idx = 0;

        for &preorder_val in &preorder[1..] {
            let node = Some(Rc::new(RefCell::new(TreeNode::new(preorder_val))));

            let last_node = stack.last().unwrap().as_ref().unwrap();
            if last_node.borrow().val != inorder[inorder_idx] {
                last_node.borrow_mut().left = node.clone();
            } else {
                let mut parent = None;
                while let Some(last) = stack.last() {
                    if last.as_ref().unwrap().borrow().val != inorder[inorder_idx] {
                        break;
                    }
                    parent = stack.pop().unwrap();
                    inorder_idx += 1;
                }
                parent.unwrap().borrow_mut().right = node.clone();
            }
            stack.push(node);
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
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        let result = Solution::build_tree(preorder, inorder);
        let result = codec.serialize(result);
        assert_eq!(result, "[3,9,20,null,null,15,7]".to_string());
    }
}
