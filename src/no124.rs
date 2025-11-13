use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = i32::MIN;
        Self::dfs(root, &mut result);
        result
    }
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if let Some(node) = node {
            let mut node = node.borrow_mut();

            let left_val = Solution::dfs(node.left.take(), result);
            let right_val = Solution::dfs(node.right.take(), result);

            *result = (*result).max(left_val + right_val + node.val);

            return 0.max(left_val.max(right_val) + node.val);
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
        let data = "[-10,9,20,null,null,15,7]".to_string();
        let root = codec.deserialize(data);
        assert_eq!(Solution::max_path_sum(root), 42);
    }
}
