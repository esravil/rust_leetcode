#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {

    pub val: i32,
    pub next: Option<Box<ListNode>>,

}

impl Solution {

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr {

            curr = node.next.take(); // curr is now the node after it, pointer is moved
            node.next = prev; // the pointer from the node.next was made null from the take method, and can now be assigned prev node, changes directions
            prev = Some(node) // prev ptr is moved to the next node, curr

        }

        // after all iteration, the prev node will be the head
        prev

    }

}

