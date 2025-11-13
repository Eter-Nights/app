use super::definition::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    fn middle_node(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head;
        let mut slow = head;

        while fast.is_some() && fast.as_ref()?.next.is_some() {
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }
        #[allow(mutable_transmutes)]
        let slow: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };
        slow.take()
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }

        let head2 = Self::middle_node(&head);

        let head = Self::sort_list(head);
        let head2 = Self::sort_list(head2);

        super::no21::Solution::merge_two_lists(head, head2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![4, 2, 1, 3]);
        let result = Solution::sort_list(head);
        assert_eq!(ListNode::transform_vec(result), vec![1, 2, 3, 4]);
    }
}
