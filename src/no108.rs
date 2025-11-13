use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }

    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;

        let mut node = TreeNode::new(nums[mid]);
        node.left = Self::helper(&nums[0..mid]);
        node.right = Self::helper(&nums[mid + 1..nums.len()]);

        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let nums = vec![-10, -3, 0, 5, 9];
        let result = Solution::sorted_array_to_bst(nums);
        let result = codec.serialize(result);
        assert_eq!(result, "[0,-3,9,-10,null,5]".to_string());
    }
}
