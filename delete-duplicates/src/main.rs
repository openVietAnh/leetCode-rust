use std::intrinsics::needs_drop;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node = head.as_mut();
    while let Some(current_node) = node {
        let mut next_node = current_node.next.take();
        while let Some(mut next) = next_node.as_mut() {
            if next.val == current_node.val {
                next_node = next.next.take();
            } else {
                current_node.next = next_node;
                break;
            }
        }
        node = current_node.next.as_mut();
    }
    return head;
}

fn main() {
    println!("Hello, world!");
}
