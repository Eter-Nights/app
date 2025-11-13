use super::definition::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            Self::check(node.borrow().left.clone(), node.borrow().right.clone())
        } else {
            true
        }
    }

    fn check(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let p = p.unwrap();
        let q = q.unwrap();

        p.borrow().val == q.borrow().val
            && Self::check(p.borrow().left.clone(), q.borrow().right.clone())
            && Self::check(p.borrow().right.clone(), q.borrow().left.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[1,2,2,3,4,4,3]".to_string();
        let root = codec.deserialize(data);
        let result = Solution::is_symmetric(root);
        assert_eq!(result, true);
    }
}
