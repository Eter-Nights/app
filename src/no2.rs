use super::definition::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut cur = &mut result;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            if let Some(n1) = l1 {
                carry += n1.val;
                l1 = n1.next;
            }
            if let Some(n2) = l2 {
                carry += n2.val;
                l2 = n2.next;
            }
            cur.next = Some(Box::new(ListNode::new(carry % 10)));
            carry /= 10;
            cur = cur.next.as_mut().unwrap();
        }

        result.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let list1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let list2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let result = Solution::add_two_numbers(list1, list2);
        assert_eq!(
            ListNode::transform_vec(result),
            vec![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }
}
