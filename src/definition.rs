#![allow(dead_code)]

use std::cell::RefCell;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl ListNode {
    pub fn from_vec(nums: Vec<i32>) -> Option<Box<ListNode>> {
        if nums.is_empty() {
            return None;
        }

        let mut head = None;
        let mut cur = &mut head;
        for val in nums {
            *cur = Some(Box::new(ListNode::new(val)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }

    pub fn transform_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut cur = &head;
        let mut result = Vec::new();

        while let Some(node) = cur {
            result.push(node.val);
            cur = &node.next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 4, 5];
        let root = ListNode::from_vec(nums);
        let result = ListNode::transform_vec(root);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
