// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {
    value: i32,
    uplink: Link,
    downlink: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
}

impl LinkedStack {
    fn new() -> Self {
        LinkedStack { head: None }
    }

    fn push(&mut self, val: i32) {
        let new_node: Box<Node> = Box::new(Node {
            value: val,
            downlink: self.head.take(),
            uplink: None,
        });

        self.head = Some(new_node);

        if let mut Some(node) = self.head {
            if let Some(other_node) = node.downlink {
                other_node.uplink = Some(node);
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        todo!();
    }
}
