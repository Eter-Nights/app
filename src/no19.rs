use super::definition::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let dummy = ListNode { val: 0, next: head };
        let mut fast = &dummy;
        let mut slow = &dummy;

        for _ in 0..n {
            fast = fast.next.as_ref()?;
        }

        while let Some(ref node) = fast.next {
            slow = slow.next.as_ref()?;
            fast = node;
        }

        #[allow(mutable_transmutes)]
        let slow: &mut ListNode = unsafe { std::mem::transmute(slow) };
        slow.next = slow.next.take()?.next;

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let result = Solution::remove_nth_from_end(head, 2);
        assert_eq!(ListNode::transform_vec(result), vec![1, 2, 3, 5]);
    }
}
