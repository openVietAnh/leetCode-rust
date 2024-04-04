#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>, 
    mut list2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    if list1.is_none() && list2.is_none() {
        return None;
    }
    let (mut node1, mut node2) = (&mut list1, &mut list2);
    let mut arr: Vec<i32> = vec![];
    while node1.is_some() {
        arr.push(node1.as_ref()?.val);
        node1 = &mut node1.as_mut()?.next;
    }
    while node2.is_some() {
        arr.push(node2.as_ref()?.val);
        node2 = &mut node2.as_mut()?.next;
    }
    arr.sort();
    let mut head = ListNode::new(*arr.last().unwrap());
    arr.pop();
    while arr.len() > 0 {
        let value = arr.pop().unwrap();
        let mut new_head = ListNode::new(value);
        new_head.next = Some(Box::new(head));
        head = new_head;
    }
    return Some(Box::new(head));
}

fn main() {
    println!("Hello, world!");
}
