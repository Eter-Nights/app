use super::definition::ListNode;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut result = None;
        let mut cur = &mut result;

        for list in lists.into_iter() {
            if let Some(node) = list {
                heap.push(Reverse(node));
            }
        }

        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.0.next.take() {
                heap.push(Reverse(next));
            }
            cur = &mut cur.insert(node.0).next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let lists = vec![
            ListNode::from_vec(vec![1, 4, 5]),
            ListNode::from_vec(vec![1, 3, 4]),
            ListNode::from_vec(vec![2, 6]),
        ];
        let result = Solution::merge_k_lists(lists);
        assert_eq!(
            ListNode::transform_vec(result),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }
}
