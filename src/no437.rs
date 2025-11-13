use super::definition::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix: HashMap<i64, i32> = HashMap::new();
        prefix.insert(0, 1);

        Self::dfs(root, 0, target_sum, &mut prefix)
    }
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut curr: i64,
        target_sum: i32,
        prefix: &mut HashMap<i64, i32>,
    ) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let mut ret = 0;
        curr += root.borrow().val as i64;

        if prefix.contains_key(&(curr - target_sum as i64)) {
            ret = *prefix.get(&(curr - target_sum as i64)).unwrap();
        }

        prefix.entry(curr).and_modify(|x| *x += 1).or_insert(1);

        ret += Self::dfs(root.borrow_mut().left.take(), curr, target_sum, prefix);
        ret += Self::dfs(root.borrow_mut().right.take(), curr, target_sum, prefix);

        prefix.entry(curr).and_modify(|x| *x -= 1);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::no297::Codec;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[10,5,-3,3,2,null,11,3,-2,null,1]".to_string();
        let root = codec.deserialize(data);
        assert_eq!(Solution::path_sum(root, 8), 3);
    }
}
