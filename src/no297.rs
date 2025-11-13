#![allow(dead_code)]

use super::definition::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::str::FromStr;

pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "[]".to_string();
        }

        let mut result = String::new();
        result.push_str("[");

        let mut deque = VecDeque::new();
        deque.push_front(root);
        while let Some(node) = deque.pop_front() {
            if let Some(node) = node {
                result.push_str(&format!("{},", node.borrow().val));
                deque.push_back(node.borrow_mut().left.take());
                deque.push_back(node.borrow_mut().right.take());
            } else {
                result.push_str("null,");
            }
        }
        while result.len() > 5 && result[result.len() - 5..result.len()].to_string() == "null," {
            result.drain(result.len() - 5..result.len());
        }
        result.pop();
        result.push_str("]");
        result
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data == "[]" {
            return None;
        }
        let vals = data[1..data.len() - 1].split(",").collect::<Vec<_>>();
        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            i32::from_str(vals[0]).unwrap(),
        ))));

        let mut i = 1;
        let mut deque = VecDeque::new();
        deque.push_front(root.clone());
        while let Some(node) = deque.pop_front() {
            if i >= vals.len() {
                break;
            }
            if vals[i] != "null" {
                node.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(
                    TreeNode::new(i32::from_str(vals[i]).unwrap()),
                )));
                deque.push_back(node.as_ref().unwrap().borrow().left.clone());
            }
            i += 1;
            if i >= vals.len() {
                break;
            }
            if vals[i] != "null" {
                node.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(
                    TreeNode::new(i32::from_str(vals[i]).unwrap()),
                )));
                deque.push_back(node.as_ref().unwrap().borrow().right.clone());
            }
            i += 1;
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let codec = Codec::new();
        let data = "[1,2,3,null,null,4,5]".to_string();
        let root = codec.deserialize(data);
        let result = codec.serialize(root);
        assert_eq!(result, "[1,2,3,null,null,4,5]");
    }
}
