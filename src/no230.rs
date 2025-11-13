use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut stack = vec![];
        let mut idx = 0;

        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                root = node.borrow().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                idx += 1;
                if idx == k {
                    return node.borrow().val;
                }
                root = node.borrow().right.clone();
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[3,1,4,null,2]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::kth_smallest(root, 1);
        assert_eq!(result, 1);
    }
}
