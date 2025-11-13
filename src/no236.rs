use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }

        let x = root.as_ref()?;
        let left = Self::lowest_common_ancestor(x.borrow_mut().left.take(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(x.borrow_mut().right.take(), p, q);

        if left.is_some() && right.is_some() {
            return root;
        }

        if left.is_some() {
            left
        } else {
            right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn test() {
        let codec = Codec::new();
        let data = "[3,5,1,6,2,0,8,null,null,7,4]".to_string();
        let root = codec.deserialize(data);
        let p = root.as_ref().unwrap().borrow().left.clone();
        let q = root.as_ref().unwrap().borrow().right.clone();

        let result = Solution::lowest_common_ancestor(root.clone(), p, q);
        assert_eq!(result, root);
    }
}
