use super::definition::ListNode;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut n| match n.next {
            Some(mut m) => {
                n.next = Self::swap_pairs(m.next);
                m.next = Some(n);
                Some(m)
            }
            None => Some(n),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let head = ListNode::from_vec(vec![1, 2, 3, 4]);
        let result = Solution::swap_pairs(head);
        assert_eq!(ListNode::transform_vec(result), vec![2, 1, 4, 3]);
    }
}
