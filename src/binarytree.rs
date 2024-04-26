use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct BTree<T: Debug> {
    head: Node<T>,
}

impl<T: Debug> Node<T> {
    fn in_order_traverse(&self, res: &mut String) -> String {
        res.push_str(&format!("{:?}", self.value).to_string());
        if self.left.is_some() {
            self.left.as_ref().unwrap().in_order_traverse(res);
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().in_order_traverse(res);
        }
        res.to_string()
    }

    fn new(arg: T) -> Node<T> {
        Node {
            value: arg,
            left: None,
            right: None,
        }
    }
}

impl<T: Debug> BTree<T> {
    fn new(head: Node<T>) -> Self {
        BTree { head }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn traverse() {
        let mut head = Node::new(1);
        let left = Node::new(2);
        let right = Node::new(3);
        head.left = Some(Box::new(left));
        head.right = Some(Box::new(right));
        let b = BTree::new(head);
        let mut res = "".to_string();
        b.head.in_order_traverse(&mut res);
        assert_eq!(res, String::from("123"))
    }
}
