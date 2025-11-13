use super::definition::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while let Some(mut node) = cur {
            cur = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        pre
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let root = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = Solution::reverse_list(root);
        assert_eq!(ListNode::transform_vec(result), vec![5, 4, 3, 2, 1]);
    }
}
